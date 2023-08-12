use quote::ToTokens;
use std::{
    env,
    fs::{self, File},
    path::Path,
};
use syn::{visit_mut::VisitMut, Expr};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <specification> <destination>", args[0]);
        std::process::exit(1);
    }

    let specification = &args[1];
    let destination = &args[2];

    let content = generate_code(specification);

    write_to_file(content, destination);

    println!("Generated code in {}", destination);
}

fn generate_code(src: &str) -> String {
    let file = File::open(src).unwrap();
    let spec = serde_json::from_reader(file).unwrap();
    let mut generator = progenitor::Generator::default();
    let tokens = generator.generate_tokens(&spec).unwrap();
    let mut ast = syn::parse2(tokens).unwrap();
    ArrayParameterVisitor.visit_file_mut(&mut ast);
    prettyplease::unparse(&ast)
}

fn write_to_file(content: String, output_file: &str) {
    let out_file = Path::new(output_file);
    fs::write(out_file, content).unwrap();
}

// This is a hack to work around the fact that progenitor doesn't support array parameters.
struct ArrayParameterVisitor;

impl VisitMut for ArrayParameterVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        *expr = self.transform_expression(expr);
        syn::visit_mut::visit_expr_mut(self, expr);
    }
}

impl ArrayParameterVisitor {
    fn transform_expression(&mut self, expr: &Expr) -> Expr {
        if let Expr::MethodCall(method_call) = expr {
            if method_call.method == "push"
                && method_call.receiver.as_ref().to_token_stream().to_string() == "query"
            {
                if let Some((key, variable)) = self.extract_key_and_variable(&method_call.args[0]) {
                    return syn::parse_quote! {
                        {
                            let mut query_values = Vec::new();
                            for carthage_array_value in *#variable {
                                query_values.push((#key, carthage_array_value.to_string()));
                            }
                            query.extend(query_values);
                        }
                    };
                }
            }
        }

        expr.clone()
    }

    fn extract_key_and_variable<'a, 'b>(
        &'a mut self,
        arg: &'b Expr,
    ) -> Option<(String, &'b Box<Expr>)> {
        if let Expr::Tuple(tuple) = arg {
            if let Expr::Lit(lit) = &tuple.elems[0] {
                if let syn::Lit::Str(s) = &lit.lit {
                    if s.value().ends_with("[]") {
                        if let Expr::MethodCall(value_method_call) = &tuple.elems[1] {
                            if value_method_call.method == "to_string" {
                                return Some((s.value(), &value_method_call.receiver));
                            }
                        }
                    }
                }
            }
        }

        None
    }
}
