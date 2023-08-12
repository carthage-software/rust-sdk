## Carthage SDK for Rust

## Installation 🚀

Install the SDK using Cargo with the following command:

```bash
cargo add carthage_rust_sdk
```

## Usage 💼

### Creating a Client

```rust
use carthage_rust_sdk::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("https://my-carthage-server.example.com");

    match client.ping().await {
        Ok(resource) => {
            println!("Server is up and running!");
            println!("Server Time: {}", resource.time);
            println!("Quote: {}", resource.quote.clone().unwrap_or_default());
        }
        Err(e) => println!("error: {}", e),
    }
}

```

### Examples

Refer to the [`examples`](./examples) directory for more examples.

## Code Of Conduct 🤝

Our community is guided by a Code of Conduct, and we expect all contributors to respect it. See the [`CODE_OF_CONDUCT`](./CODE_OF_CONDUCT.md) for more details.

## Contributing 🎁

The Carthage SDK for Rust thrives on contributions from the open-source community. We value every contribution, no matter how small.

## License 📜

The Carthage SDK for Rust is distributed under the MIT License. See [`LICENSE`](./LICENSE) for more information.

---

We hope you enjoy using the Carthage SDK for Rust! For any queries or suggestions, don't hesitate to open an issue or submit a pull request. Happy coding!
