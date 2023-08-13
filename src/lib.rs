#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCollect {
        pub collect_logs: Vec<LogManagementCollectCollectLogsItem>,
    }
    impl From<&LogManagementCollect> for LogManagementCollect {
        fn from(value: &LogManagementCollect) -> Self {
            value.clone()
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCollectBody {
        pub collect_logs: Vec<LogManagementCollectBodyCollectLogsItem>,
    }
    impl From<&LogManagementCollectBody> for LogManagementCollectBody {
        fn from(value: &LogManagementCollectBody) -> Self {
            value.clone()
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCollectBodyCollectLogsItem {
        pub entries: Vec<LogManagementCollectBodyCollectLogsItemEntriesItem>,
        pub log: LogManagementCollectBodyCollectLogsItemLog,
    }
    impl From<&LogManagementCollectBodyCollectLogsItem> for LogManagementCollectBodyCollectLogsItem {
        fn from(value: &LogManagementCollectBodyCollectLogsItem) -> Self {
            value.clone()
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCollectBodyCollectLogsItemEntriesItem {
        pub attributes: serde_json::Map<String, serde_json::Value>,
        pub context: serde_json::Map<String, serde_json::Value>,
        ///The timestamp indicating when the entry occurred.
        pub occurred_at: chrono::DateTime<chrono::offset::Utc>,
        ///The source of the entry, indicating what part of the system the entry is from.
        pub source: LogManagementCollectBodyCollectLogsItemEntriesItemSource,
        pub tags: Vec<String>,
    }
    impl From<&LogManagementCollectBodyCollectLogsItemEntriesItem>
        for LogManagementCollectBodyCollectLogsItemEntriesItem
    {
        fn from(value: &LogManagementCollectBodyCollectLogsItemEntriesItem) -> Self {
            value.clone()
        }
    }
    ///The source of the entry, indicating what part of the system the entry is from.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCollectBodyCollectLogsItemEntriesItemSource(String);
    impl std::ops::Deref for LogManagementCollectBodyCollectLogsItemEntriesItemSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCollectBodyCollectLogsItemEntriesItemSource> for String {
        fn from(value: LogManagementCollectBodyCollectLogsItemEntriesItemSource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCollectBodyCollectLogsItemEntriesItemSource>
        for LogManagementCollectBodyCollectLogsItemEntriesItemSource
    {
        fn from(value: &LogManagementCollectBodyCollectLogsItemEntriesItemSource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCollectBodyCollectLogsItemEntriesItemSource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 128usize {
                return Err("longer than 128 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCollectBodyCollectLogsItemEntriesItemSource {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCollectBodyCollectLogsItemEntriesItemSource {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCollectBodyCollectLogsItemEntriesItemSource {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCollectBodyCollectLogsItemEntriesItemSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCollectBodyCollectLogsItemLog {
        ///The severity level of the log.
        pub level: LogManagementCollectBodyCollectLogsItemLogLevel,
        ///The namespace associated with the log.
        pub namespace: LogManagementCollectBodyCollectLogsItemLogNamespace,
        ///The template used for formatting the log message.
        pub template: LogManagementCollectBodyCollectLogsItemLogTemplate,
    }
    impl From<&LogManagementCollectBodyCollectLogsItemLog>
        for LogManagementCollectBodyCollectLogsItemLog
    {
        fn from(value: &LogManagementCollectBodyCollectLogsItemLog) -> Self {
            value.clone()
        }
    }
    ///The severity level of the log.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementCollectBodyCollectLogsItemLogLevel(i64);
    impl std::ops::Deref for LogManagementCollectBodyCollectLogsItemLogLevel {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementCollectBodyCollectLogsItemLogLevel> for i64 {
        fn from(value: LogManagementCollectBodyCollectLogsItemLogLevel) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCollectBodyCollectLogsItemLogLevel>
        for LogManagementCollectBodyCollectLogsItemLogLevel
    {
        fn from(value: &LogManagementCollectBodyCollectLogsItemLogLevel) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64> for LogManagementCollectBodyCollectLogsItemLogLevel {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCollectBodyCollectLogsItemLogLevel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The namespace associated with the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCollectBodyCollectLogsItemLogNamespace(String);
    impl std::ops::Deref for LogManagementCollectBodyCollectLogsItemLogNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCollectBodyCollectLogsItemLogNamespace> for String {
        fn from(value: LogManagementCollectBodyCollectLogsItemLogNamespace) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCollectBodyCollectLogsItemLogNamespace>
        for LogManagementCollectBodyCollectLogsItemLogNamespace
    {
        fn from(value: &LogManagementCollectBodyCollectLogsItemLogNamespace) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCollectBodyCollectLogsItemLogNamespace {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 64usize {
                return Err("longer than 64 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCollectBodyCollectLogsItemLogNamespace {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCollectBodyCollectLogsItemLogNamespace {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCollectBodyCollectLogsItemLogNamespace {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCollectBodyCollectLogsItemLogNamespace {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The template used for formatting the log message.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCollectBodyCollectLogsItemLogTemplate(String);
    impl std::ops::Deref for LogManagementCollectBodyCollectLogsItemLogTemplate {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCollectBodyCollectLogsItemLogTemplate> for String {
        fn from(value: LogManagementCollectBodyCollectLogsItemLogTemplate) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCollectBodyCollectLogsItemLogTemplate>
        for LogManagementCollectBodyCollectLogsItemLogTemplate
    {
        fn from(value: &LogManagementCollectBodyCollectLogsItemLogTemplate) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCollectBodyCollectLogsItemLogTemplate {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 4096usize {
                return Err("longer than 4096 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCollectBodyCollectLogsItemLogTemplate {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCollectBodyCollectLogsItemLogTemplate {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCollectBodyCollectLogsItemLogTemplate {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCollectBodyCollectLogsItemLogTemplate {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCollectCollectLogsItem {
        pub entries: Vec<LogManagementCollectCollectLogsItemEntriesItem>,
        pub log: LogManagementCollectCollectLogsItemLog,
    }
    impl From<&LogManagementCollectCollectLogsItem> for LogManagementCollectCollectLogsItem {
        fn from(value: &LogManagementCollectCollectLogsItem) -> Self {
            value.clone()
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCollectCollectLogsItemEntriesItem {
        pub attributes: serde_json::Map<String, serde_json::Value>,
        pub context: serde_json::Map<String, serde_json::Value>,
        ///The timestamp indicating when the entry occurred.
        pub occurred_at: chrono::DateTime<chrono::offset::Utc>,
        ///The source of the entry, indicating what part of the system the entry is from.
        pub source: LogManagementCollectCollectLogsItemEntriesItemSource,
        pub tags: Vec<String>,
    }
    impl From<&LogManagementCollectCollectLogsItemEntriesItem>
        for LogManagementCollectCollectLogsItemEntriesItem
    {
        fn from(value: &LogManagementCollectCollectLogsItemEntriesItem) -> Self {
            value.clone()
        }
    }
    ///The source of the entry, indicating what part of the system the entry is from.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCollectCollectLogsItemEntriesItemSource(String);
    impl std::ops::Deref for LogManagementCollectCollectLogsItemEntriesItemSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCollectCollectLogsItemEntriesItemSource> for String {
        fn from(value: LogManagementCollectCollectLogsItemEntriesItemSource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCollectCollectLogsItemEntriesItemSource>
        for LogManagementCollectCollectLogsItemEntriesItemSource
    {
        fn from(value: &LogManagementCollectCollectLogsItemEntriesItemSource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCollectCollectLogsItemEntriesItemSource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 128usize {
                return Err("longer than 128 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCollectCollectLogsItemEntriesItemSource {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCollectCollectLogsItemEntriesItemSource {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCollectCollectLogsItemEntriesItemSource {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCollectCollectLogsItemEntriesItemSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCollectCollectLogsItemLog {
        ///The severity level of the log.
        pub level: LogManagementCollectCollectLogsItemLogLevel,
        ///The namespace associated with the log.
        pub namespace: LogManagementCollectCollectLogsItemLogNamespace,
        ///The template used for formatting the log message.
        pub template: LogManagementCollectCollectLogsItemLogTemplate,
    }
    impl From<&LogManagementCollectCollectLogsItemLog> for LogManagementCollectCollectLogsItemLog {
        fn from(value: &LogManagementCollectCollectLogsItemLog) -> Self {
            value.clone()
        }
    }
    ///The severity level of the log.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementCollectCollectLogsItemLogLevel(i64);
    impl std::ops::Deref for LogManagementCollectCollectLogsItemLogLevel {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementCollectCollectLogsItemLogLevel> for i64 {
        fn from(value: LogManagementCollectCollectLogsItemLogLevel) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCollectCollectLogsItemLogLevel>
        for LogManagementCollectCollectLogsItemLogLevel
    {
        fn from(value: &LogManagementCollectCollectLogsItemLogLevel) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64> for LogManagementCollectCollectLogsItemLogLevel {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCollectCollectLogsItemLogLevel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The namespace associated with the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCollectCollectLogsItemLogNamespace(String);
    impl std::ops::Deref for LogManagementCollectCollectLogsItemLogNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCollectCollectLogsItemLogNamespace> for String {
        fn from(value: LogManagementCollectCollectLogsItemLogNamespace) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCollectCollectLogsItemLogNamespace>
        for LogManagementCollectCollectLogsItemLogNamespace
    {
        fn from(value: &LogManagementCollectCollectLogsItemLogNamespace) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCollectCollectLogsItemLogNamespace {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 64usize {
                return Err("longer than 64 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCollectCollectLogsItemLogNamespace {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCollectCollectLogsItemLogNamespace {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCollectCollectLogsItemLogNamespace {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCollectCollectLogsItemLogNamespace {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The template used for formatting the log message.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCollectCollectLogsItemLogTemplate(String);
    impl std::ops::Deref for LogManagementCollectCollectLogsItemLogTemplate {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCollectCollectLogsItemLogTemplate> for String {
        fn from(value: LogManagementCollectCollectLogsItemLogTemplate) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCollectCollectLogsItemLogTemplate>
        for LogManagementCollectCollectLogsItemLogTemplate
    {
        fn from(value: &LogManagementCollectCollectLogsItemLogTemplate) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCollectCollectLogsItemLogTemplate {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 4096usize {
                return Err("longer than 4096 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCollectCollectLogsItemLogTemplate {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCollectCollectLogsItemLogTemplate {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCollectCollectLogsItemLogTemplate {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCollectCollectLogsItemLogTemplate {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCollectLogBody {
        pub entries: Vec<LogManagementCollectLogBodyEntriesItem>,
        pub log: LogManagementCollectLogBodyLog,
    }
    impl From<&LogManagementCollectLogBody> for LogManagementCollectLogBody {
        fn from(value: &LogManagementCollectLogBody) -> Self {
            value.clone()
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCollectLogBodyEntriesItem {
        pub attributes: serde_json::Map<String, serde_json::Value>,
        pub context: serde_json::Map<String, serde_json::Value>,
        ///The timestamp indicating when the entry occurred.
        pub occurred_at: chrono::DateTime<chrono::offset::Utc>,
        ///The source of the entry, indicating what part of the system the entry is from.
        pub source: LogManagementCollectLogBodyEntriesItemSource,
        pub tags: Vec<String>,
    }
    impl From<&LogManagementCollectLogBodyEntriesItem> for LogManagementCollectLogBodyEntriesItem {
        fn from(value: &LogManagementCollectLogBodyEntriesItem) -> Self {
            value.clone()
        }
    }
    ///The source of the entry, indicating what part of the system the entry is from.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCollectLogBodyEntriesItemSource(String);
    impl std::ops::Deref for LogManagementCollectLogBodyEntriesItemSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCollectLogBodyEntriesItemSource> for String {
        fn from(value: LogManagementCollectLogBodyEntriesItemSource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCollectLogBodyEntriesItemSource>
        for LogManagementCollectLogBodyEntriesItemSource
    {
        fn from(value: &LogManagementCollectLogBodyEntriesItemSource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCollectLogBodyEntriesItemSource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 128usize {
                return Err("longer than 128 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCollectLogBodyEntriesItemSource {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCollectLogBodyEntriesItemSource {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCollectLogBodyEntriesItemSource {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCollectLogBodyEntriesItemSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCollectLogBodyLog {
        ///The severity level of the log.
        pub level: LogManagementCollectLogBodyLogLevel,
        ///The namespace associated with the log.
        pub namespace: LogManagementCollectLogBodyLogNamespace,
        ///The template used for formatting the log message.
        pub template: LogManagementCollectLogBodyLogTemplate,
    }
    impl From<&LogManagementCollectLogBodyLog> for LogManagementCollectLogBodyLog {
        fn from(value: &LogManagementCollectLogBodyLog) -> Self {
            value.clone()
        }
    }
    ///The severity level of the log.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementCollectLogBodyLogLevel(i64);
    impl std::ops::Deref for LogManagementCollectLogBodyLogLevel {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementCollectLogBodyLogLevel> for i64 {
        fn from(value: LogManagementCollectLogBodyLogLevel) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCollectLogBodyLogLevel> for LogManagementCollectLogBodyLogLevel {
        fn from(value: &LogManagementCollectLogBodyLogLevel) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64> for LogManagementCollectLogBodyLogLevel {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCollectLogBodyLogLevel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The namespace associated with the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCollectLogBodyLogNamespace(String);
    impl std::ops::Deref for LogManagementCollectLogBodyLogNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCollectLogBodyLogNamespace> for String {
        fn from(value: LogManagementCollectLogBodyLogNamespace) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCollectLogBodyLogNamespace> for LogManagementCollectLogBodyLogNamespace {
        fn from(value: &LogManagementCollectLogBodyLogNamespace) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCollectLogBodyLogNamespace {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 64usize {
                return Err("longer than 64 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCollectLogBodyLogNamespace {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCollectLogBodyLogNamespace {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCollectLogBodyLogNamespace {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCollectLogBodyLogNamespace {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The template used for formatting the log message.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCollectLogBodyLogTemplate(String);
    impl std::ops::Deref for LogManagementCollectLogBodyLogTemplate {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCollectLogBodyLogTemplate> for String {
        fn from(value: LogManagementCollectLogBodyLogTemplate) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCollectLogBodyLogTemplate> for LogManagementCollectLogBodyLogTemplate {
        fn from(value: &LogManagementCollectLogBodyLogTemplate) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCollectLogBodyLogTemplate {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 4096usize {
                return Err("longer than 4096 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCollectLogBodyLogTemplate {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCollectLogBodyLogTemplate {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCollectLogBodyLogTemplate {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCollectLogBodyLogTemplate {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCreateLogBody {
        ///The severity level of the log.
        pub level: LogManagementCreateLogBodyLevel,
        ///The namespace associated with the log.
        pub namespace: LogManagementCreateLogBodyNamespace,
        ///The template used for formatting the log message.
        pub template: LogManagementCreateLogBodyTemplate,
    }
    impl From<&LogManagementCreateLogBody> for LogManagementCreateLogBody {
        fn from(value: &LogManagementCreateLogBody) -> Self {
            value.clone()
        }
    }
    ///The severity level of the log.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementCreateLogBodyLevel(i64);
    impl std::ops::Deref for LogManagementCreateLogBodyLevel {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementCreateLogBodyLevel> for i64 {
        fn from(value: LogManagementCreateLogBodyLevel) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCreateLogBodyLevel> for LogManagementCreateLogBodyLevel {
        fn from(value: &LogManagementCreateLogBodyLevel) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64> for LogManagementCreateLogBodyLevel {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCreateLogBodyLevel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The namespace associated with the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCreateLogBodyNamespace(String);
    impl std::ops::Deref for LogManagementCreateLogBodyNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCreateLogBodyNamespace> for String {
        fn from(value: LogManagementCreateLogBodyNamespace) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCreateLogBodyNamespace> for LogManagementCreateLogBodyNamespace {
        fn from(value: &LogManagementCreateLogBodyNamespace) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCreateLogBodyNamespace {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 64usize {
                return Err("longer than 64 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCreateLogBodyNamespace {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCreateLogBodyNamespace {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCreateLogBodyNamespace {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCreateLogBodyNamespace {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The template used for formatting the log message.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCreateLogBodyTemplate(String);
    impl std::ops::Deref for LogManagementCreateLogBodyTemplate {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCreateLogBodyTemplate> for String {
        fn from(value: LogManagementCreateLogBodyTemplate) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCreateLogBodyTemplate> for LogManagementCreateLogBodyTemplate {
        fn from(value: &LogManagementCreateLogBodyTemplate) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCreateLogBodyTemplate {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 4096usize {
                return Err("longer than 4096 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCreateLogBodyTemplate {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCreateLogBodyTemplate {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCreateLogBodyTemplate {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCreateLogBodyTemplate {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCreateLogEntryBody {
        pub attributes: serde_json::Map<String, serde_json::Value>,
        pub context: serde_json::Map<String, serde_json::Value>,
        ///The identity of the associated log.
        pub log_identity: uuid::Uuid,
        ///The timestamp indicating when the entry occurred.
        pub occurred_at: chrono::DateTime<chrono::offset::Utc>,
        ///The source of the entry, indicating what part of the system the entry is from.
        pub source: LogManagementCreateLogEntryBodySource,
        pub tags: Vec<String>,
    }
    impl From<&LogManagementCreateLogEntryBody> for LogManagementCreateLogEntryBody {
        fn from(value: &LogManagementCreateLogEntryBody) -> Self {
            value.clone()
        }
    }
    ///The source of the entry, indicating what part of the system the entry is from.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCreateLogEntryBodySource(String);
    impl std::ops::Deref for LogManagementCreateLogEntryBodySource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCreateLogEntryBodySource> for String {
        fn from(value: LogManagementCreateLogEntryBodySource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCreateLogEntryBodySource> for LogManagementCreateLogEntryBodySource {
        fn from(value: &LogManagementCreateLogEntryBodySource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCreateLogEntryBodySource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 128usize {
                return Err("longer than 128 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCreateLogEntryBodySource {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCreateLogEntryBodySource {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCreateLogEntryBodySource {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCreateLogEntryBodySource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Resource representing a log entry.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCreateLogEntryResponse {
        pub attributes: serde_json::Map<String, serde_json::Value>,
        pub context: serde_json::Map<String, serde_json::Value>,
        ///Timestamp when the log entry was created.
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        ///Unique identity of the log entry.
        pub identity: uuid::Uuid,
        ///Identity of the associated log.
        pub log_identity: uuid::Uuid,
        ///Timestamp when the log entry occurred.
        pub occurred_at: chrono::DateTime<chrono::offset::Utc>,
        ///Source of the log entry.
        pub source: LogManagementCreateLogEntryResponseSource,
        ///Tags associated with the log entry.
        pub tags: Vec<LogManagementCreateLogEntryResponseTagsItem>,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementCreateLogEntryResponseType,
        ///Timestamp when the log entry was last updated.
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }
    impl From<&LogManagementCreateLogEntryResponse> for LogManagementCreateLogEntryResponse {
        fn from(value: &LogManagementCreateLogEntryResponse) -> Self {
            value.clone()
        }
    }
    ///Source of the log entry.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCreateLogEntryResponseSource(String);
    impl std::ops::Deref for LogManagementCreateLogEntryResponseSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCreateLogEntryResponseSource> for String {
        fn from(value: LogManagementCreateLogEntryResponseSource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCreateLogEntryResponseSource>
        for LogManagementCreateLogEntryResponseSource
    {
        fn from(value: &LogManagementCreateLogEntryResponseSource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCreateLogEntryResponseSource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCreateLogEntryResponseSource {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCreateLogEntryResponseSource {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCreateLogEntryResponseSource {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCreateLogEntryResponseSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCreateLogEntryResponseTagsItem(String);
    impl std::ops::Deref for LogManagementCreateLogEntryResponseTagsItem {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCreateLogEntryResponseTagsItem> for String {
        fn from(value: LogManagementCreateLogEntryResponseTagsItem) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCreateLogEntryResponseTagsItem>
        for LogManagementCreateLogEntryResponseTagsItem
    {
        fn from(value: &LogManagementCreateLogEntryResponseTagsItem) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCreateLogEntryResponseTagsItem {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCreateLogEntryResponseTagsItem {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCreateLogEntryResponseTagsItem {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCreateLogEntryResponseTagsItem {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCreateLogEntryResponseTagsItem {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementCreateLogEntryResponseType {
        #[serde(rename = "log_entry")]
        LogEntry,
    }
    impl From<&LogManagementCreateLogEntryResponseType> for LogManagementCreateLogEntryResponseType {
        fn from(value: &LogManagementCreateLogEntryResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementCreateLogEntryResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntry => "log_entry".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementCreateLogEntryResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry" => Ok(Self::LogEntry),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCreateLogEntryResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCreateLogEntryResponseType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCreateLogEntryResponseType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Resource representing a log.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCreateLogResponse {
        ///Timestamp when the log was created.
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        ///Timestamp of the first entry occurrence, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub first_entry_occurred_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        ///Unique identity of the log.
        pub identity: uuid::Uuid,
        ///Timestamp of the last entry occurrence, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub last_entry_occurred_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub level: LogManagementCreateLogResponseLevel,
        ///The namespace of the log.
        pub namespace: LogManagementCreateLogResponseNamespace,
        ///The template for formatting the message.
        pub template: LogManagementCreateLogResponseTemplate,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementCreateLogResponseType,
        ///Timestamp when the log was last updated.
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }
    impl From<&LogManagementCreateLogResponse> for LogManagementCreateLogResponse {
        fn from(value: &LogManagementCreateLogResponse) -> Self {
            value.clone()
        }
    }
    ///The log level.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementCreateLogResponseLevel {
        ///Name of the log level.
        pub name: LogManagementCreateLogResponseLevelName,
        ///Value of the log level.
        pub value: LogManagementCreateLogResponseLevelValue,
    }
    impl From<&LogManagementCreateLogResponseLevel> for LogManagementCreateLogResponseLevel {
        fn from(value: &LogManagementCreateLogResponseLevel) -> Self {
            value.clone()
        }
    }
    ///Name of the log level.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementCreateLogResponseLevelName {
        Debug,
        Info,
        Notice,
        Warning,
        Error,
        Critical,
        Alert,
        Emergency,
    }
    impl From<&LogManagementCreateLogResponseLevelName> for LogManagementCreateLogResponseLevelName {
        fn from(value: &LogManagementCreateLogResponseLevelName) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementCreateLogResponseLevelName {
        fn to_string(&self) -> String {
            match *self {
                Self::Debug => "Debug".to_string(),
                Self::Info => "Info".to_string(),
                Self::Notice => "Notice".to_string(),
                Self::Warning => "Warning".to_string(),
                Self::Error => "Error".to_string(),
                Self::Critical => "Critical".to_string(),
                Self::Alert => "Alert".to_string(),
                Self::Emergency => "Emergency".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementCreateLogResponseLevelName {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "Debug" => Ok(Self::Debug),
                "Info" => Ok(Self::Info),
                "Notice" => Ok(Self::Notice),
                "Warning" => Ok(Self::Warning),
                "Error" => Ok(Self::Error),
                "Critical" => Ok(Self::Critical),
                "Alert" => Ok(Self::Alert),
                "Emergency" => Ok(Self::Emergency),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCreateLogResponseLevelName {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCreateLogResponseLevelName {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCreateLogResponseLevelName {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Value of the log level.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementCreateLogResponseLevelValue(i64);
    impl std::ops::Deref for LogManagementCreateLogResponseLevelValue {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementCreateLogResponseLevelValue> for i64 {
        fn from(value: LogManagementCreateLogResponseLevelValue) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCreateLogResponseLevelValue> for LogManagementCreateLogResponseLevelValue {
        fn from(value: &LogManagementCreateLogResponseLevelValue) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64> for LogManagementCreateLogResponseLevelValue {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCreateLogResponseLevelValue {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The namespace of the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCreateLogResponseNamespace(String);
    impl std::ops::Deref for LogManagementCreateLogResponseNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCreateLogResponseNamespace> for String {
        fn from(value: LogManagementCreateLogResponseNamespace) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCreateLogResponseNamespace> for LogManagementCreateLogResponseNamespace {
        fn from(value: &LogManagementCreateLogResponseNamespace) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCreateLogResponseNamespace {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCreateLogResponseNamespace {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCreateLogResponseNamespace {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCreateLogResponseNamespace {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCreateLogResponseNamespace {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The template for formatting the message.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementCreateLogResponseTemplate(String);
    impl std::ops::Deref for LogManagementCreateLogResponseTemplate {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementCreateLogResponseTemplate> for String {
        fn from(value: LogManagementCreateLogResponseTemplate) -> Self {
            value.0
        }
    }
    impl From<&LogManagementCreateLogResponseTemplate> for LogManagementCreateLogResponseTemplate {
        fn from(value: &LogManagementCreateLogResponseTemplate) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementCreateLogResponseTemplate {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCreateLogResponseTemplate {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCreateLogResponseTemplate {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCreateLogResponseTemplate {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementCreateLogResponseTemplate {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementCreateLogResponseType {
        #[serde(rename = "log")]
        Log,
    }
    impl From<&LogManagementCreateLogResponseType> for LogManagementCreateLogResponseType {
        fn from(value: &LogManagementCreateLogResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementCreateLogResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::Log => "log".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementCreateLogResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log" => Ok(Self::Log),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementCreateLogResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementCreateLogResponseType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementCreateLogResponseType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementGetLogCollectionLevelsItem(i64);
    impl std::ops::Deref for LogManagementGetLogCollectionLevelsItem {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementGetLogCollectionLevelsItem> for i64 {
        fn from(value: LogManagementGetLogCollectionLevelsItem) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogCollectionLevelsItem> for LogManagementGetLogCollectionLevelsItem {
        fn from(value: &LogManagementGetLogCollectionLevelsItem) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64> for LogManagementGetLogCollectionLevelsItem {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementGetLogCollectionLevelsItem {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The order of the logs.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogCollectionOrder {
        #[serde(rename = "ASC")]
        Asc,
        #[serde(rename = "DESC")]
        Desc,
    }
    impl From<&LogManagementGetLogCollectionOrder> for LogManagementGetLogCollectionOrder {
        fn from(value: &LogManagementGetLogCollectionOrder) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogCollectionOrder {
        fn to_string(&self) -> String {
            match *self {
                Self::Asc => "ASC".to_string(),
                Self::Desc => "DESC".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogCollectionOrder {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "ASC" => Ok(Self::Asc),
                "DESC" => Ok(Self::Desc),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogCollectionOrder {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogCollectionOrder {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogCollectionOrder {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl Default for LogManagementGetLogCollectionOrder {
        fn default() -> Self {
            LogManagementGetLogCollectionOrder::Desc
        }
    }
    ///Paginated collection of LogResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogCollectionResponse {
        ///First item index.
        pub first: i64,
        ///Array of LogResource.
        pub items: Vec<LogManagementGetLogCollectionResponseItemsItem>,
        ///Number of items per page.
        pub items_per_page: i64,
        ///Last item index.
        pub last: i64,
        ///Next page number, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next: Option<i64>,
        ///Current page number.
        pub page: i64,
        ///Previous page number, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub previous: Option<i64>,
        ///Total number of items.
        pub total_items: i64,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogCollectionResponseType,
    }
    impl From<&LogManagementGetLogCollectionResponse> for LogManagementGetLogCollectionResponse {
        fn from(value: &LogManagementGetLogCollectionResponse) -> Self {
            value.clone()
        }
    }
    ///Resource representing a log.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogCollectionResponseItemsItem {
        ///Timestamp when the log was created.
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        ///Timestamp of the first entry occurrence, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub first_entry_occurred_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        ///Unique identity of the log.
        pub identity: uuid::Uuid,
        ///Timestamp of the last entry occurrence, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub last_entry_occurred_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub level: LogManagementGetLogCollectionResponseItemsItemLevel,
        ///The namespace of the log.
        pub namespace: LogManagementGetLogCollectionResponseItemsItemNamespace,
        ///The template for formatting the message.
        pub template: LogManagementGetLogCollectionResponseItemsItemTemplate,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogCollectionResponseItemsItemType,
        ///Timestamp when the log was last updated.
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }
    impl From<&LogManagementGetLogCollectionResponseItemsItem>
        for LogManagementGetLogCollectionResponseItemsItem
    {
        fn from(value: &LogManagementGetLogCollectionResponseItemsItem) -> Self {
            value.clone()
        }
    }
    ///The log level.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogCollectionResponseItemsItemLevel {
        ///Name of the log level.
        pub name: LogManagementGetLogCollectionResponseItemsItemLevelName,
        ///Value of the log level.
        pub value: LogManagementGetLogCollectionResponseItemsItemLevelValue,
    }
    impl From<&LogManagementGetLogCollectionResponseItemsItemLevel>
        for LogManagementGetLogCollectionResponseItemsItemLevel
    {
        fn from(value: &LogManagementGetLogCollectionResponseItemsItemLevel) -> Self {
            value.clone()
        }
    }
    ///Name of the log level.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogCollectionResponseItemsItemLevelName {
        Debug,
        Info,
        Notice,
        Warning,
        Error,
        Critical,
        Alert,
        Emergency,
    }
    impl From<&LogManagementGetLogCollectionResponseItemsItemLevelName>
        for LogManagementGetLogCollectionResponseItemsItemLevelName
    {
        fn from(value: &LogManagementGetLogCollectionResponseItemsItemLevelName) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogCollectionResponseItemsItemLevelName {
        fn to_string(&self) -> String {
            match *self {
                Self::Debug => "Debug".to_string(),
                Self::Info => "Info".to_string(),
                Self::Notice => "Notice".to_string(),
                Self::Warning => "Warning".to_string(),
                Self::Error => "Error".to_string(),
                Self::Critical => "Critical".to_string(),
                Self::Alert => "Alert".to_string(),
                Self::Emergency => "Emergency".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogCollectionResponseItemsItemLevelName {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "Debug" => Ok(Self::Debug),
                "Info" => Ok(Self::Info),
                "Notice" => Ok(Self::Notice),
                "Warning" => Ok(Self::Warning),
                "Error" => Ok(Self::Error),
                "Critical" => Ok(Self::Critical),
                "Alert" => Ok(Self::Alert),
                "Emergency" => Ok(Self::Emergency),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogCollectionResponseItemsItemLevelName {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogCollectionResponseItemsItemLevelName {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogCollectionResponseItemsItemLevelName {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Value of the log level.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementGetLogCollectionResponseItemsItemLevelValue(i64);
    impl std::ops::Deref for LogManagementGetLogCollectionResponseItemsItemLevelValue {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementGetLogCollectionResponseItemsItemLevelValue> for i64 {
        fn from(value: LogManagementGetLogCollectionResponseItemsItemLevelValue) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogCollectionResponseItemsItemLevelValue>
        for LogManagementGetLogCollectionResponseItemsItemLevelValue
    {
        fn from(value: &LogManagementGetLogCollectionResponseItemsItemLevelValue) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64> for LogManagementGetLogCollectionResponseItemsItemLevelValue {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementGetLogCollectionResponseItemsItemLevelValue {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The namespace of the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementGetLogCollectionResponseItemsItemNamespace(String);
    impl std::ops::Deref for LogManagementGetLogCollectionResponseItemsItemNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementGetLogCollectionResponseItemsItemNamespace> for String {
        fn from(value: LogManagementGetLogCollectionResponseItemsItemNamespace) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogCollectionResponseItemsItemNamespace>
        for LogManagementGetLogCollectionResponseItemsItemNamespace
    {
        fn from(value: &LogManagementGetLogCollectionResponseItemsItemNamespace) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementGetLogCollectionResponseItemsItemNamespace {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogCollectionResponseItemsItemNamespace {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogCollectionResponseItemsItemNamespace {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogCollectionResponseItemsItemNamespace {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementGetLogCollectionResponseItemsItemNamespace {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The template for formatting the message.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementGetLogCollectionResponseItemsItemTemplate(String);
    impl std::ops::Deref for LogManagementGetLogCollectionResponseItemsItemTemplate {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementGetLogCollectionResponseItemsItemTemplate> for String {
        fn from(value: LogManagementGetLogCollectionResponseItemsItemTemplate) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogCollectionResponseItemsItemTemplate>
        for LogManagementGetLogCollectionResponseItemsItemTemplate
    {
        fn from(value: &LogManagementGetLogCollectionResponseItemsItemTemplate) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementGetLogCollectionResponseItemsItemTemplate {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogCollectionResponseItemsItemTemplate {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogCollectionResponseItemsItemTemplate {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogCollectionResponseItemsItemTemplate {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementGetLogCollectionResponseItemsItemTemplate {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogCollectionResponseItemsItemType {
        #[serde(rename = "log")]
        Log,
    }
    impl From<&LogManagementGetLogCollectionResponseItemsItemType>
        for LogManagementGetLogCollectionResponseItemsItemType
    {
        fn from(value: &LogManagementGetLogCollectionResponseItemsItemType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogCollectionResponseItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::Log => "log".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogCollectionResponseItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log" => Ok(Self::Log),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogCollectionResponseItemsItemType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogCollectionResponseItemsItemType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogCollectionResponseItemsItemType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogCollectionResponseType {
        #[serde(rename = "paginated_collection")]
        PaginatedCollection,
    }
    impl From<&LogManagementGetLogCollectionResponseType>
        for LogManagementGetLogCollectionResponseType
    {
        fn from(value: &LogManagementGetLogCollectionResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogCollectionResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::PaginatedCollection => "paginated_collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogCollectionResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "paginated_collection" => Ok(Self::PaginatedCollection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///The field to sort the logs by.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogCollectionSortBy {
        #[serde(rename = "createdAt")]
        CreatedAt,
        #[serde(rename = "updatedAt")]
        UpdatedAt,
        #[serde(rename = "firstEntryOccurredAt")]
        FirstEntryOccurredAt,
        #[serde(rename = "lastEntryOccurredAt")]
        LastEntryOccurredAt,
        #[serde(rename = "level")]
        Level,
    }
    impl From<&LogManagementGetLogCollectionSortBy> for LogManagementGetLogCollectionSortBy {
        fn from(value: &LogManagementGetLogCollectionSortBy) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogCollectionSortBy {
        fn to_string(&self) -> String {
            match *self {
                Self::CreatedAt => "createdAt".to_string(),
                Self::UpdatedAt => "updatedAt".to_string(),
                Self::FirstEntryOccurredAt => "firstEntryOccurredAt".to_string(),
                Self::LastEntryOccurredAt => "lastEntryOccurredAt".to_string(),
                Self::Level => "level".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogCollectionSortBy {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "createdAt" => Ok(Self::CreatedAt),
                "updatedAt" => Ok(Self::UpdatedAt),
                "firstEntryOccurredAt" => Ok(Self::FirstEntryOccurredAt),
                "lastEntryOccurredAt" => Ok(Self::LastEntryOccurredAt),
                "level" => Ok(Self::Level),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogCollectionSortBy {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogCollectionSortBy {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogCollectionSortBy {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl Default for LogManagementGetLogCollectionSortBy {
        fn default() -> Self {
            LogManagementGetLogCollectionSortBy::LastEntryOccurredAt
        }
    }
    ///The order of the log entries.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntryCollectionOrder {
        #[serde(rename = "ASC")]
        Asc,
        #[serde(rename = "DESC")]
        Desc,
    }
    impl From<&LogManagementGetLogEntryCollectionOrder> for LogManagementGetLogEntryCollectionOrder {
        fn from(value: &LogManagementGetLogEntryCollectionOrder) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntryCollectionOrder {
        fn to_string(&self) -> String {
            match *self {
                Self::Asc => "ASC".to_string(),
                Self::Desc => "DESC".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryCollectionOrder {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "ASC" => Ok(Self::Asc),
                "DESC" => Ok(Self::Desc),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryCollectionOrder {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogEntryCollectionOrder {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogEntryCollectionOrder {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl Default for LogManagementGetLogEntryCollectionOrder {
        fn default() -> Self {
            LogManagementGetLogEntryCollectionOrder::Desc
        }
    }
    ///Paginated collection of LogEntryResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogEntryCollectionResponse {
        ///First item index.
        pub first: i64,
        ///Array of LogEntryResource.
        pub items: Vec<LogManagementGetLogEntryCollectionResponseItemsItem>,
        ///Number of items per page.
        pub items_per_page: i64,
        ///Last item index.
        pub last: i64,
        ///Next page number, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next: Option<i64>,
        ///Current page number.
        pub page: i64,
        ///Previous page number, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub previous: Option<i64>,
        ///Total number of items.
        pub total_items: i64,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogEntryCollectionResponseType,
    }
    impl From<&LogManagementGetLogEntryCollectionResponse>
        for LogManagementGetLogEntryCollectionResponse
    {
        fn from(value: &LogManagementGetLogEntryCollectionResponse) -> Self {
            value.clone()
        }
    }
    ///Resource representing a log entry.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogEntryCollectionResponseItemsItem {
        pub attributes: serde_json::Map<String, serde_json::Value>,
        pub context: serde_json::Map<String, serde_json::Value>,
        ///Timestamp when the log entry was created.
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        ///Unique identity of the log entry.
        pub identity: uuid::Uuid,
        ///Identity of the associated log.
        pub log_identity: uuid::Uuid,
        ///Timestamp when the log entry occurred.
        pub occurred_at: chrono::DateTime<chrono::offset::Utc>,
        ///Source of the log entry.
        pub source: LogManagementGetLogEntryCollectionResponseItemsItemSource,
        ///Tags associated with the log entry.
        pub tags: Vec<LogManagementGetLogEntryCollectionResponseItemsItemTagsItem>,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogEntryCollectionResponseItemsItemType,
        ///Timestamp when the log entry was last updated.
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }
    impl From<&LogManagementGetLogEntryCollectionResponseItemsItem>
        for LogManagementGetLogEntryCollectionResponseItemsItem
    {
        fn from(value: &LogManagementGetLogEntryCollectionResponseItemsItem) -> Self {
            value.clone()
        }
    }
    ///Source of the log entry.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementGetLogEntryCollectionResponseItemsItemSource(String);
    impl std::ops::Deref for LogManagementGetLogEntryCollectionResponseItemsItemSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementGetLogEntryCollectionResponseItemsItemSource> for String {
        fn from(value: LogManagementGetLogEntryCollectionResponseItemsItemSource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogEntryCollectionResponseItemsItemSource>
        for LogManagementGetLogEntryCollectionResponseItemsItemSource
    {
        fn from(value: &LogManagementGetLogEntryCollectionResponseItemsItemSource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryCollectionResponseItemsItemSource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryCollectionResponseItemsItemSource {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogEntryCollectionResponseItemsItemSource {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogEntryCollectionResponseItemsItemSource {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementGetLogEntryCollectionResponseItemsItemSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementGetLogEntryCollectionResponseItemsItemTagsItem(String);
    impl std::ops::Deref for LogManagementGetLogEntryCollectionResponseItemsItemTagsItem {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementGetLogEntryCollectionResponseItemsItemTagsItem> for String {
        fn from(value: LogManagementGetLogEntryCollectionResponseItemsItemTagsItem) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogEntryCollectionResponseItemsItemTagsItem>
        for LogManagementGetLogEntryCollectionResponseItemsItemTagsItem
    {
        fn from(value: &LogManagementGetLogEntryCollectionResponseItemsItemTagsItem) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryCollectionResponseItemsItemTagsItem {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryCollectionResponseItemsItemTagsItem {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogEntryCollectionResponseItemsItemTagsItem
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogEntryCollectionResponseItemsItemTagsItem {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementGetLogEntryCollectionResponseItemsItemTagsItem {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntryCollectionResponseItemsItemType {
        #[serde(rename = "log_entry")]
        LogEntry,
    }
    impl From<&LogManagementGetLogEntryCollectionResponseItemsItemType>
        for LogManagementGetLogEntryCollectionResponseItemsItemType
    {
        fn from(value: &LogManagementGetLogEntryCollectionResponseItemsItemType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntryCollectionResponseItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntry => "log_entry".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryCollectionResponseItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry" => Ok(Self::LogEntry),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryCollectionResponseItemsItemType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogEntryCollectionResponseItemsItemType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogEntryCollectionResponseItemsItemType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntryCollectionResponseType {
        #[serde(rename = "paginated_collection")]
        PaginatedCollection,
    }
    impl From<&LogManagementGetLogEntryCollectionResponseType>
        for LogManagementGetLogEntryCollectionResponseType
    {
        fn from(value: &LogManagementGetLogEntryCollectionResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntryCollectionResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::PaginatedCollection => "paginated_collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryCollectionResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "paginated_collection" => Ok(Self::PaginatedCollection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogEntryCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogEntryCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///The frequency for log statistics, specifying how the data should be aggregated.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntryFrequencyCountCollectionFrequency {
        #[serde(rename = "yearly")]
        Yearly,
        #[serde(rename = "quarterly")]
        Quarterly,
        #[serde(rename = "monthly")]
        Monthly,
        #[serde(rename = "weekly")]
        Weekly,
        #[serde(rename = "daily")]
        Daily,
        #[serde(rename = "hourly")]
        Hourly,
    }
    impl From<&LogManagementGetLogEntryFrequencyCountCollectionFrequency>
        for LogManagementGetLogEntryFrequencyCountCollectionFrequency
    {
        fn from(value: &LogManagementGetLogEntryFrequencyCountCollectionFrequency) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntryFrequencyCountCollectionFrequency {
        fn to_string(&self) -> String {
            match *self {
                Self::Yearly => "yearly".to_string(),
                Self::Quarterly => "quarterly".to_string(),
                Self::Monthly => "monthly".to_string(),
                Self::Weekly => "weekly".to_string(),
                Self::Daily => "daily".to_string(),
                Self::Hourly => "hourly".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryFrequencyCountCollectionFrequency {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "yearly" => Ok(Self::Yearly),
                "quarterly" => Ok(Self::Quarterly),
                "monthly" => Ok(Self::Monthly),
                "weekly" => Ok(Self::Weekly),
                "daily" => Ok(Self::Daily),
                "hourly" => Ok(Self::Hourly),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryFrequencyCountCollectionFrequency {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogEntryFrequencyCountCollectionFrequency {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogEntryFrequencyCountCollectionFrequency {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Collection of LogEntryFrequencyCountResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogEntryFrequencyCountCollectionResponse {
        ///Array of LogEntryFrequencyCountResource.
        pub items: Vec<LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItem>,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogEntryFrequencyCountCollectionResponseType,
    }
    impl From<&LogManagementGetLogEntryFrequencyCountCollectionResponse>
        for LogManagementGetLogEntryFrequencyCountCollectionResponse
    {
        fn from(value: &LogManagementGetLogEntryFrequencyCountCollectionResponse) -> Self {
            value.clone()
        }
    }
    ///Resource representing the frequency count of log entries.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItem {
        ///Count of log entries for the specified date.
        pub count: std::num::NonZeroU64,
        ///Date associated with the frequency count.
        pub date: chrono::DateTime<chrono::offset::Utc>,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItemType,
    }
    impl From<&LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItem>
        for LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItem
    {
        fn from(value: &LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItem) -> Self {
            value.clone()
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItemType {
        #[serde(rename = "log_entry_frequency_count")]
        LogEntryFrequencyCount,
    }
    impl From<&LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItemType>
        for LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItemType
    {
        fn from(
            value: &LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItemType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntryFrequencyCount => "log_entry_frequency_count".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_frequency_count" => Ok(Self::LogEntryFrequencyCount),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogEntryFrequencyCountCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntryFrequencyCountCollectionResponseType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementGetLogEntryFrequencyCountCollectionResponseType>
        for LogManagementGetLogEntryFrequencyCountCollectionResponseType
    {
        fn from(value: &LogManagementGetLogEntryFrequencyCountCollectionResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntryFrequencyCountCollectionResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryFrequencyCountCollectionResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryFrequencyCountCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogEntryFrequencyCountCollectionResponseType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogEntryFrequencyCountCollectionResponseType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Resource representing a log entry.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogEntryResponse {
        pub attributes: serde_json::Map<String, serde_json::Value>,
        pub context: serde_json::Map<String, serde_json::Value>,
        ///Timestamp when the log entry was created.
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        ///Unique identity of the log entry.
        pub identity: uuid::Uuid,
        ///Identity of the associated log.
        pub log_identity: uuid::Uuid,
        ///Timestamp when the log entry occurred.
        pub occurred_at: chrono::DateTime<chrono::offset::Utc>,
        ///Source of the log entry.
        pub source: LogManagementGetLogEntryResponseSource,
        ///Tags associated with the log entry.
        pub tags: Vec<LogManagementGetLogEntryResponseTagsItem>,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogEntryResponseType,
        ///Timestamp when the log entry was last updated.
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }
    impl From<&LogManagementGetLogEntryResponse> for LogManagementGetLogEntryResponse {
        fn from(value: &LogManagementGetLogEntryResponse) -> Self {
            value.clone()
        }
    }
    ///Source of the log entry.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementGetLogEntryResponseSource(String);
    impl std::ops::Deref for LogManagementGetLogEntryResponseSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementGetLogEntryResponseSource> for String {
        fn from(value: LogManagementGetLogEntryResponseSource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogEntryResponseSource> for LogManagementGetLogEntryResponseSource {
        fn from(value: &LogManagementGetLogEntryResponseSource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryResponseSource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryResponseSource {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogEntryResponseSource {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogEntryResponseSource {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementGetLogEntryResponseSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementGetLogEntryResponseTagsItem(String);
    impl std::ops::Deref for LogManagementGetLogEntryResponseTagsItem {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementGetLogEntryResponseTagsItem> for String {
        fn from(value: LogManagementGetLogEntryResponseTagsItem) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogEntryResponseTagsItem> for LogManagementGetLogEntryResponseTagsItem {
        fn from(value: &LogManagementGetLogEntryResponseTagsItem) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryResponseTagsItem {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryResponseTagsItem {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogEntryResponseTagsItem {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogEntryResponseTagsItem {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementGetLogEntryResponseTagsItem {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntryResponseType {
        #[serde(rename = "log_entry")]
        LogEntry,
    }
    impl From<&LogManagementGetLogEntryResponseType> for LogManagementGetLogEntryResponseType {
        fn from(value: &LogManagementGetLogEntryResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntryResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntry => "log_entry".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry" => Ok(Self::LogEntry),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogEntryResponseType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogEntryResponseType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Collection of LogEntrySourceResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogEntrySourceCollectionResponse {
        ///Array of LogEntrySourceResource.
        pub items: Vec<LogManagementGetLogEntrySourceCollectionResponseItemsItem>,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogEntrySourceCollectionResponseType,
    }
    impl From<&LogManagementGetLogEntrySourceCollectionResponse>
        for LogManagementGetLogEntrySourceCollectionResponse
    {
        fn from(value: &LogManagementGetLogEntrySourceCollectionResponse) -> Self {
            value.clone()
        }
    }
    ///Resource representing a log entry source.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogEntrySourceCollectionResponseItemsItem {
        ///Source of the log entry.
        pub source: LogManagementGetLogEntrySourceCollectionResponseItemsItemSource,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogEntrySourceCollectionResponseItemsItemType,
    }
    impl From<&LogManagementGetLogEntrySourceCollectionResponseItemsItem>
        for LogManagementGetLogEntrySourceCollectionResponseItemsItem
    {
        fn from(value: &LogManagementGetLogEntrySourceCollectionResponseItemsItem) -> Self {
            value.clone()
        }
    }
    ///Source of the log entry.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementGetLogEntrySourceCollectionResponseItemsItemSource(String);
    impl std::ops::Deref for LogManagementGetLogEntrySourceCollectionResponseItemsItemSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementGetLogEntrySourceCollectionResponseItemsItemSource> for String {
        fn from(value: LogManagementGetLogEntrySourceCollectionResponseItemsItemSource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogEntrySourceCollectionResponseItemsItemSource>
        for LogManagementGetLogEntrySourceCollectionResponseItemsItemSource
    {
        fn from(value: &LogManagementGetLogEntrySourceCollectionResponseItemsItemSource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntrySourceCollectionResponseItemsItemSource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementGetLogEntrySourceCollectionResponseItemsItemSource
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogEntrySourceCollectionResponseItemsItemSource
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogEntrySourceCollectionResponseItemsItemSource
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementGetLogEntrySourceCollectionResponseItemsItemSource
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntrySourceCollectionResponseItemsItemType {
        #[serde(rename = "log_entry_source")]
        LogEntrySource,
    }
    impl From<&LogManagementGetLogEntrySourceCollectionResponseItemsItemType>
        for LogManagementGetLogEntrySourceCollectionResponseItemsItemType
    {
        fn from(value: &LogManagementGetLogEntrySourceCollectionResponseItemsItemType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntrySourceCollectionResponseItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntrySource => "log_entry_source".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntrySourceCollectionResponseItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_source" => Ok(Self::LogEntrySource),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntrySourceCollectionResponseItemsItemType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogEntrySourceCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogEntrySourceCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntrySourceCollectionResponseType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementGetLogEntrySourceCollectionResponseType>
        for LogManagementGetLogEntrySourceCollectionResponseType
    {
        fn from(value: &LogManagementGetLogEntrySourceCollectionResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntrySourceCollectionResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntrySourceCollectionResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntrySourceCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogEntrySourceCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogEntrySourceCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Collection of LogEntrySourceFrequencyResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogEntrySourceFrequencyCollectionResponse {
        ///Array of LogEntrySourceFrequencyResource.
        pub items: Vec<LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItem>,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogEntrySourceFrequencyCollectionResponseType,
    }
    impl From<&LogManagementGetLogEntrySourceFrequencyCollectionResponse>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponse
    {
        fn from(value: &LogManagementGetLogEntrySourceFrequencyCollectionResponse) -> Self {
            value.clone()
        }
    }
    ///Resource representing the frequency count of log entries by source.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItem {
        ///Count of log entries for the specified source.
        pub count: std::num::NonZeroU64,
        pub percentage: f64,
        ///Source associated with the frequency count.
        pub source: LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemSource,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemType,
    }
    impl From<&LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItem>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItem
    {
        fn from(
            value: &LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItem,
        ) -> Self {
            value.clone()
        }
    }
    ///Source associated with the frequency count.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemSource(String);
    impl std::ops::Deref for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemSource> for String {
        fn from(
            value: LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemSource,
        ) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemSource>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemSource
    {
        fn from(
            value: &LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemSource,
        ) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemSource
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 128usize {
                return Err("longer than 128 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemSource
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemSource
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemSource
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemSource
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemType {
        #[serde(rename = "log_entry_source_frequency")]
        LogEntrySourceFrequency,
    }
    impl From<&LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemType>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemType
    {
        fn from(
            value: &LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntrySourceFrequency => "log_entry_source_frequency".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_source_frequency" => Ok(Self::LogEntrySourceFrequency),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntrySourceFrequencyCollectionResponseType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementGetLogEntrySourceFrequencyCollectionResponseType>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseType
    {
        fn from(value: &LogManagementGetLogEntrySourceFrequencyCollectionResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntrySourceFrequencyCollectionResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntrySourceFrequencyCollectionResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntrySourceFrequencyCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogEntrySourceFrequencyCollectionResponseType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Collection of LogEntryTagResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogEntryTagCollectionResponse {
        ///Array of LogEntryTagResource.
        pub items: Vec<LogManagementGetLogEntryTagCollectionResponseItemsItem>,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogEntryTagCollectionResponseType,
    }
    impl From<&LogManagementGetLogEntryTagCollectionResponse>
        for LogManagementGetLogEntryTagCollectionResponse
    {
        fn from(value: &LogManagementGetLogEntryTagCollectionResponse) -> Self {
            value.clone()
        }
    }
    ///Resource representing a log entry tag.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogEntryTagCollectionResponseItemsItem {
        ///Tag of the log entry.
        pub tag: LogManagementGetLogEntryTagCollectionResponseItemsItemTag,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogEntryTagCollectionResponseItemsItemType,
    }
    impl From<&LogManagementGetLogEntryTagCollectionResponseItemsItem>
        for LogManagementGetLogEntryTagCollectionResponseItemsItem
    {
        fn from(value: &LogManagementGetLogEntryTagCollectionResponseItemsItem) -> Self {
            value.clone()
        }
    }
    ///Tag of the log entry.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementGetLogEntryTagCollectionResponseItemsItemTag(String);
    impl std::ops::Deref for LogManagementGetLogEntryTagCollectionResponseItemsItemTag {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementGetLogEntryTagCollectionResponseItemsItemTag> for String {
        fn from(value: LogManagementGetLogEntryTagCollectionResponseItemsItemTag) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogEntryTagCollectionResponseItemsItemTag>
        for LogManagementGetLogEntryTagCollectionResponseItemsItemTag
    {
        fn from(value: &LogManagementGetLogEntryTagCollectionResponseItemsItemTag) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryTagCollectionResponseItemsItemTag {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryTagCollectionResponseItemsItemTag {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogEntryTagCollectionResponseItemsItemTag {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogEntryTagCollectionResponseItemsItemTag {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementGetLogEntryTagCollectionResponseItemsItemTag {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntryTagCollectionResponseItemsItemType {
        #[serde(rename = "log_entry_tag")]
        LogEntryTag,
    }
    impl From<&LogManagementGetLogEntryTagCollectionResponseItemsItemType>
        for LogManagementGetLogEntryTagCollectionResponseItemsItemType
    {
        fn from(value: &LogManagementGetLogEntryTagCollectionResponseItemsItemType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntryTagCollectionResponseItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntryTag => "log_entry_tag".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryTagCollectionResponseItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_tag" => Ok(Self::LogEntryTag),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryTagCollectionResponseItemsItemType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogEntryTagCollectionResponseItemsItemType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogEntryTagCollectionResponseItemsItemType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntryTagCollectionResponseType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementGetLogEntryTagCollectionResponseType>
        for LogManagementGetLogEntryTagCollectionResponseType
    {
        fn from(value: &LogManagementGetLogEntryTagCollectionResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntryTagCollectionResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryTagCollectionResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryTagCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogEntryTagCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogEntryTagCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Collection of LogEntryTagDistributionResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogEntryTagDistributionCollectionResponse {
        ///Array of LogEntryTagDistributionResource.
        pub items: Vec<LogManagementGetLogEntryTagDistributionCollectionResponseItemsItem>,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogEntryTagDistributionCollectionResponseType,
    }
    impl From<&LogManagementGetLogEntryTagDistributionCollectionResponse>
        for LogManagementGetLogEntryTagDistributionCollectionResponse
    {
        fn from(value: &LogManagementGetLogEntryTagDistributionCollectionResponse) -> Self {
            value.clone()
        }
    }
    ///Resource representing the distribution of log entries by tag.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogEntryTagDistributionCollectionResponseItemsItem {
        ///Count of log entries for the specified tag.
        pub count: std::num::NonZeroU64,
        ///Tag associated with the distribution.
        pub tag: LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemTag,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemType,
    }
    impl From<&LogManagementGetLogEntryTagDistributionCollectionResponseItemsItem>
        for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItem
    {
        fn from(
            value: &LogManagementGetLogEntryTagDistributionCollectionResponseItemsItem,
        ) -> Self {
            value.clone()
        }
    }
    ///Tag associated with the distribution.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemTag(String);
    impl std::ops::Deref for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemTag {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemTag> for String {
        fn from(
            value: LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemTag,
        ) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemTag>
        for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemTag
    {
        fn from(
            value: &LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemTag,
        ) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemTag {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemTag
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemTag
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemTag
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemTag
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemType {
        #[serde(rename = "log_entry_tag_distribution")]
        LogEntryTagDistribution,
    }
    impl From<&LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemType>
        for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemType
    {
        fn from(
            value: &LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntryTagDistribution => "log_entry_tag_distribution".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_tag_distribution" => Ok(Self::LogEntryTagDistribution),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogEntryTagDistributionCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogEntryTagDistributionCollectionResponseType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementGetLogEntryTagDistributionCollectionResponseType>
        for LogManagementGetLogEntryTagDistributionCollectionResponseType
    {
        fn from(value: &LogManagementGetLogEntryTagDistributionCollectionResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogEntryTagDistributionCollectionResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogEntryTagDistributionCollectionResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogEntryTagDistributionCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogEntryTagDistributionCollectionResponseType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogEntryTagDistributionCollectionResponseType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///The frequency for log statistics, specifying how the data should be aggregated.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogFrequencyCountCollectionFrequency {
        #[serde(rename = "yearly")]
        Yearly,
        #[serde(rename = "quarterly")]
        Quarterly,
        #[serde(rename = "monthly")]
        Monthly,
        #[serde(rename = "weekly")]
        Weekly,
        #[serde(rename = "daily")]
        Daily,
        #[serde(rename = "hourly")]
        Hourly,
    }
    impl From<&LogManagementGetLogFrequencyCountCollectionFrequency>
        for LogManagementGetLogFrequencyCountCollectionFrequency
    {
        fn from(value: &LogManagementGetLogFrequencyCountCollectionFrequency) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogFrequencyCountCollectionFrequency {
        fn to_string(&self) -> String {
            match *self {
                Self::Yearly => "yearly".to_string(),
                Self::Quarterly => "quarterly".to_string(),
                Self::Monthly => "monthly".to_string(),
                Self::Weekly => "weekly".to_string(),
                Self::Daily => "daily".to_string(),
                Self::Hourly => "hourly".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogFrequencyCountCollectionFrequency {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "yearly" => Ok(Self::Yearly),
                "quarterly" => Ok(Self::Quarterly),
                "monthly" => Ok(Self::Monthly),
                "weekly" => Ok(Self::Weekly),
                "daily" => Ok(Self::Daily),
                "hourly" => Ok(Self::Hourly),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogFrequencyCountCollectionFrequency {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogFrequencyCountCollectionFrequency {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogFrequencyCountCollectionFrequency {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Collection of LogFrequencyCountResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogFrequencyCountCollectionResponse {
        ///Array of LogFrequencyCountResource.
        pub items: Vec<LogManagementGetLogFrequencyCountCollectionResponseItemsItem>,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogFrequencyCountCollectionResponseType,
    }
    impl From<&LogManagementGetLogFrequencyCountCollectionResponse>
        for LogManagementGetLogFrequencyCountCollectionResponse
    {
        fn from(value: &LogManagementGetLogFrequencyCountCollectionResponse) -> Self {
            value.clone()
        }
    }
    ///Resource representing the frequency count of logs.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogFrequencyCountCollectionResponseItemsItem {
        ///Count of logs for the specified date.
        pub count: std::num::NonZeroU64,
        ///Date associated with the frequency count.
        pub date: chrono::DateTime<chrono::offset::Utc>,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogFrequencyCountCollectionResponseItemsItemType,
    }
    impl From<&LogManagementGetLogFrequencyCountCollectionResponseItemsItem>
        for LogManagementGetLogFrequencyCountCollectionResponseItemsItem
    {
        fn from(value: &LogManagementGetLogFrequencyCountCollectionResponseItemsItem) -> Self {
            value.clone()
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogFrequencyCountCollectionResponseItemsItemType {
        #[serde(rename = "log_frequency_count")]
        LogFrequencyCount,
    }
    impl From<&LogManagementGetLogFrequencyCountCollectionResponseItemsItemType>
        for LogManagementGetLogFrequencyCountCollectionResponseItemsItemType
    {
        fn from(value: &LogManagementGetLogFrequencyCountCollectionResponseItemsItemType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogFrequencyCountCollectionResponseItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogFrequencyCount => "log_frequency_count".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogFrequencyCountCollectionResponseItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_frequency_count" => Ok(Self::LogFrequencyCount),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementGetLogFrequencyCountCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogFrequencyCountCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogFrequencyCountCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogFrequencyCountCollectionResponseType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementGetLogFrequencyCountCollectionResponseType>
        for LogManagementGetLogFrequencyCountCollectionResponseType
    {
        fn from(value: &LogManagementGetLogFrequencyCountCollectionResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogFrequencyCountCollectionResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogFrequencyCountCollectionResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogFrequencyCountCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogFrequencyCountCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogFrequencyCountCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Collection of LogLevelStatisticsResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogLevelStatisticsCollectionResponse {
        ///Array of LogLevelStatisticsResource.
        pub items: Vec<LogManagementGetLogLevelStatisticsCollectionResponseItemsItem>,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogLevelStatisticsCollectionResponseType,
    }
    impl From<&LogManagementGetLogLevelStatisticsCollectionResponse>
        for LogManagementGetLogLevelStatisticsCollectionResponse
    {
        fn from(value: &LogManagementGetLogLevelStatisticsCollectionResponse) -> Self {
            value.clone()
        }
    }
    ///Resource representing the statistics of logs by level.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogLevelStatisticsCollectionResponseItemsItem {
        ///Count of logs for the specified level.
        pub count: std::num::NonZeroU64,
        pub level: LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevel,
        pub percentage: f64,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogLevelStatisticsCollectionResponseItemsItemType,
    }
    impl From<&LogManagementGetLogLevelStatisticsCollectionResponseItemsItem>
        for LogManagementGetLogLevelStatisticsCollectionResponseItemsItem
    {
        fn from(value: &LogManagementGetLogLevelStatisticsCollectionResponseItemsItem) -> Self {
            value.clone()
        }
    }
    ///Level associated with the statistics.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevel {
        ///Name of the level.
        pub name: LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelName,
        ///Value of the level.
        pub value: LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelValue,
    }
    impl From<&LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevel>
        for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevel
    {
        fn from(
            value: &LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevel,
        ) -> Self {
            value.clone()
        }
    }
    ///Name of the level.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelName {
        Debug,
        Info,
        Notice,
        Warning,
        Error,
        Critical,
        Alert,
        Emergency,
    }
    impl From<&LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelName>
        for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelName
    {
        fn from(
            value: &LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelName,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelName {
        fn to_string(&self) -> String {
            match *self {
                Self::Debug => "Debug".to_string(),
                Self::Info => "Info".to_string(),
                Self::Notice => "Notice".to_string(),
                Self::Warning => "Warning".to_string(),
                Self::Error => "Error".to_string(),
                Self::Critical => "Critical".to_string(),
                Self::Alert => "Alert".to_string(),
                Self::Emergency => "Emergency".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelName {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "Debug" => Ok(Self::Debug),
                "Info" => Ok(Self::Info),
                "Notice" => Ok(Self::Notice),
                "Warning" => Ok(Self::Warning),
                "Error" => Ok(Self::Error),
                "Critical" => Ok(Self::Critical),
                "Alert" => Ok(Self::Alert),
                "Emergency" => Ok(Self::Emergency),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelName
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelName
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelName
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Value of the level.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelValue(i64);
    impl std::ops::Deref for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelValue {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelValue> for i64 {
        fn from(
            value: LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelValue,
        ) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelValue>
        for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelValue
    {
        fn from(
            value: &LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelValue,
        ) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64>
        for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelValue
    {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemLevelValue
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogLevelStatisticsCollectionResponseItemsItemType {
        #[serde(rename = "log_level_statistics")]
        LogLevelStatistics,
    }
    impl From<&LogManagementGetLogLevelStatisticsCollectionResponseItemsItemType>
        for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemType
    {
        fn from(value: &LogManagementGetLogLevelStatisticsCollectionResponseItemsItemType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogLevelStatistics => "log_level_statistics".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_level_statistics" => Ok(Self::LogLevelStatistics),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogLevelStatisticsCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogLevelStatisticsCollectionResponseType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementGetLogLevelStatisticsCollectionResponseType>
        for LogManagementGetLogLevelStatisticsCollectionResponseType
    {
        fn from(value: &LogManagementGetLogLevelStatisticsCollectionResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogLevelStatisticsCollectionResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogLevelStatisticsCollectionResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogLevelStatisticsCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogLevelStatisticsCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogLevelStatisticsCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Collection of LogNamespaceResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogNamespaceCollectionResponse {
        ///Array of LogNamespaceResource.
        pub items: Vec<LogManagementGetLogNamespaceCollectionResponseItemsItem>,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogNamespaceCollectionResponseType,
    }
    impl From<&LogManagementGetLogNamespaceCollectionResponse>
        for LogManagementGetLogNamespaceCollectionResponse
    {
        fn from(value: &LogManagementGetLogNamespaceCollectionResponse) -> Self {
            value.clone()
        }
    }
    ///Resource representing a log namespace.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogNamespaceCollectionResponseItemsItem {
        ///Namespace of the log.
        pub namespace: LogManagementGetLogNamespaceCollectionResponseItemsItemNamespace,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogNamespaceCollectionResponseItemsItemType,
    }
    impl From<&LogManagementGetLogNamespaceCollectionResponseItemsItem>
        for LogManagementGetLogNamespaceCollectionResponseItemsItem
    {
        fn from(value: &LogManagementGetLogNamespaceCollectionResponseItemsItem) -> Self {
            value.clone()
        }
    }
    ///Namespace of the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementGetLogNamespaceCollectionResponseItemsItemNamespace(String);
    impl std::ops::Deref for LogManagementGetLogNamespaceCollectionResponseItemsItemNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementGetLogNamespaceCollectionResponseItemsItemNamespace> for String {
        fn from(value: LogManagementGetLogNamespaceCollectionResponseItemsItemNamespace) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogNamespaceCollectionResponseItemsItemNamespace>
        for LogManagementGetLogNamespaceCollectionResponseItemsItemNamespace
    {
        fn from(value: &LogManagementGetLogNamespaceCollectionResponseItemsItemNamespace) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementGetLogNamespaceCollectionResponseItemsItemNamespace {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementGetLogNamespaceCollectionResponseItemsItemNamespace
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogNamespaceCollectionResponseItemsItemNamespace
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementGetLogNamespaceCollectionResponseItemsItemNamespace
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementGetLogNamespaceCollectionResponseItemsItemNamespace
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogNamespaceCollectionResponseItemsItemType {
        #[serde(rename = "log_namespace")]
        LogNamespace,
    }
    impl From<&LogManagementGetLogNamespaceCollectionResponseItemsItemType>
        for LogManagementGetLogNamespaceCollectionResponseItemsItemType
    {
        fn from(value: &LogManagementGetLogNamespaceCollectionResponseItemsItemType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogNamespaceCollectionResponseItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogNamespace => "log_namespace".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogNamespaceCollectionResponseItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_namespace" => Ok(Self::LogNamespace),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogNamespaceCollectionResponseItemsItemType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementGetLogNamespaceCollectionResponseItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogNamespaceCollectionResponseItemsItemType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogNamespaceCollectionResponseType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementGetLogNamespaceCollectionResponseType>
        for LogManagementGetLogNamespaceCollectionResponseType
    {
        fn from(value: &LogManagementGetLogNamespaceCollectionResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogNamespaceCollectionResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogNamespaceCollectionResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogNamespaceCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogNamespaceCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogNamespaceCollectionResponseType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Resource representing a log.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogResponse {
        ///Timestamp when the log was created.
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        ///Timestamp of the first entry occurrence, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub first_entry_occurred_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        ///Unique identity of the log.
        pub identity: uuid::Uuid,
        ///Timestamp of the last entry occurrence, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub last_entry_occurred_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub level: LogManagementGetLogResponseLevel,
        ///The namespace of the log.
        pub namespace: LogManagementGetLogResponseNamespace,
        ///The template for formatting the message.
        pub template: LogManagementGetLogResponseTemplate,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementGetLogResponseType,
        ///Timestamp when the log was last updated.
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }
    impl From<&LogManagementGetLogResponse> for LogManagementGetLogResponse {
        fn from(value: &LogManagementGetLogResponse) -> Self {
            value.clone()
        }
    }
    ///The log level.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementGetLogResponseLevel {
        ///Name of the log level.
        pub name: LogManagementGetLogResponseLevelName,
        ///Value of the log level.
        pub value: LogManagementGetLogResponseLevelValue,
    }
    impl From<&LogManagementGetLogResponseLevel> for LogManagementGetLogResponseLevel {
        fn from(value: &LogManagementGetLogResponseLevel) -> Self {
            value.clone()
        }
    }
    ///Name of the log level.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogResponseLevelName {
        Debug,
        Info,
        Notice,
        Warning,
        Error,
        Critical,
        Alert,
        Emergency,
    }
    impl From<&LogManagementGetLogResponseLevelName> for LogManagementGetLogResponseLevelName {
        fn from(value: &LogManagementGetLogResponseLevelName) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogResponseLevelName {
        fn to_string(&self) -> String {
            match *self {
                Self::Debug => "Debug".to_string(),
                Self::Info => "Info".to_string(),
                Self::Notice => "Notice".to_string(),
                Self::Warning => "Warning".to_string(),
                Self::Error => "Error".to_string(),
                Self::Critical => "Critical".to_string(),
                Self::Alert => "Alert".to_string(),
                Self::Emergency => "Emergency".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogResponseLevelName {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "Debug" => Ok(Self::Debug),
                "Info" => Ok(Self::Info),
                "Notice" => Ok(Self::Notice),
                "Warning" => Ok(Self::Warning),
                "Error" => Ok(Self::Error),
                "Critical" => Ok(Self::Critical),
                "Alert" => Ok(Self::Alert),
                "Emergency" => Ok(Self::Emergency),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogResponseLevelName {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogResponseLevelName {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogResponseLevelName {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Value of the log level.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementGetLogResponseLevelValue(i64);
    impl std::ops::Deref for LogManagementGetLogResponseLevelValue {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementGetLogResponseLevelValue> for i64 {
        fn from(value: LogManagementGetLogResponseLevelValue) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogResponseLevelValue> for LogManagementGetLogResponseLevelValue {
        fn from(value: &LogManagementGetLogResponseLevelValue) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64> for LogManagementGetLogResponseLevelValue {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementGetLogResponseLevelValue {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The namespace of the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementGetLogResponseNamespace(String);
    impl std::ops::Deref for LogManagementGetLogResponseNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementGetLogResponseNamespace> for String {
        fn from(value: LogManagementGetLogResponseNamespace) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogResponseNamespace> for LogManagementGetLogResponseNamespace {
        fn from(value: &LogManagementGetLogResponseNamespace) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementGetLogResponseNamespace {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogResponseNamespace {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogResponseNamespace {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogResponseNamespace {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementGetLogResponseNamespace {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The template for formatting the message.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementGetLogResponseTemplate(String);
    impl std::ops::Deref for LogManagementGetLogResponseTemplate {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementGetLogResponseTemplate> for String {
        fn from(value: LogManagementGetLogResponseTemplate) -> Self {
            value.0
        }
    }
    impl From<&LogManagementGetLogResponseTemplate> for LogManagementGetLogResponseTemplate {
        fn from(value: &LogManagementGetLogResponseTemplate) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementGetLogResponseTemplate {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogResponseTemplate {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogResponseTemplate {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogResponseTemplate {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementGetLogResponseTemplate {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementGetLogResponseType {
        #[serde(rename = "log")]
        Log,
    }
    impl From<&LogManagementGetLogResponseType> for LogManagementGetLogResponseType {
        fn from(value: &LogManagementGetLogResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementGetLogResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::Log => "log".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementGetLogResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log" => Ok(Self::Log),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementGetLogResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementGetLogResponseType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementGetLogResponseType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogCollectLog {
        pub entries: Vec<LogManagementLogCollectLogEntriesItem>,
        pub log: LogManagementLogCollectLogLog,
    }
    impl From<&LogManagementLogCollectLog> for LogManagementLogCollectLog {
        fn from(value: &LogManagementLogCollectLog) -> Self {
            value.clone()
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogCollectLogEntriesItem {
        pub attributes: serde_json::Map<String, serde_json::Value>,
        pub context: serde_json::Map<String, serde_json::Value>,
        ///The timestamp indicating when the entry occurred.
        pub occurred_at: chrono::DateTime<chrono::offset::Utc>,
        ///The source of the entry, indicating what part of the system the entry is from.
        pub source: LogManagementLogCollectLogEntriesItemSource,
        pub tags: Vec<String>,
    }
    impl From<&LogManagementLogCollectLogEntriesItem> for LogManagementLogCollectLogEntriesItem {
        fn from(value: &LogManagementLogCollectLogEntriesItem) -> Self {
            value.clone()
        }
    }
    ///The source of the entry, indicating what part of the system the entry is from.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogCollectLogEntriesItemSource(String);
    impl std::ops::Deref for LogManagementLogCollectLogEntriesItemSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogCollectLogEntriesItemSource> for String {
        fn from(value: LogManagementLogCollectLogEntriesItemSource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogCollectLogEntriesItemSource>
        for LogManagementLogCollectLogEntriesItemSource
    {
        fn from(value: &LogManagementLogCollectLogEntriesItemSource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogCollectLogEntriesItemSource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 128usize {
                return Err("longer than 128 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogCollectLogEntriesItemSource {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogCollectLogEntriesItemSource {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogCollectLogEntriesItemSource {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogCollectLogEntriesItemSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogCollectLogEntry {
        pub attributes: serde_json::Map<String, serde_json::Value>,
        pub context: serde_json::Map<String, serde_json::Value>,
        ///The timestamp indicating when the entry occurred.
        pub occurred_at: chrono::DateTime<chrono::offset::Utc>,
        ///The source of the entry, indicating what part of the system the entry is from.
        pub source: LogManagementLogCollectLogEntrySource,
        pub tags: Vec<String>,
    }
    impl From<&LogManagementLogCollectLogEntry> for LogManagementLogCollectLogEntry {
        fn from(value: &LogManagementLogCollectLogEntry) -> Self {
            value.clone()
        }
    }
    ///The source of the entry, indicating what part of the system the entry is from.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogCollectLogEntrySource(String);
    impl std::ops::Deref for LogManagementLogCollectLogEntrySource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogCollectLogEntrySource> for String {
        fn from(value: LogManagementLogCollectLogEntrySource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogCollectLogEntrySource> for LogManagementLogCollectLogEntrySource {
        fn from(value: &LogManagementLogCollectLogEntrySource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogCollectLogEntrySource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 128usize {
                return Err("longer than 128 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogCollectLogEntrySource {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogCollectLogEntrySource {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogCollectLogEntrySource {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogCollectLogEntrySource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogCollectLogLog {
        ///The severity level of the log.
        pub level: LogManagementLogCollectLogLogLevel,
        ///The namespace associated with the log.
        pub namespace: LogManagementLogCollectLogLogNamespace,
        ///The template used for formatting the log message.
        pub template: LogManagementLogCollectLogLogTemplate,
    }
    impl From<&LogManagementLogCollectLogLog> for LogManagementLogCollectLogLog {
        fn from(value: &LogManagementLogCollectLogLog) -> Self {
            value.clone()
        }
    }
    ///The severity level of the log.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementLogCollectLogLogLevel(i64);
    impl std::ops::Deref for LogManagementLogCollectLogLogLevel {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementLogCollectLogLogLevel> for i64 {
        fn from(value: LogManagementLogCollectLogLogLevel) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogCollectLogLogLevel> for LogManagementLogCollectLogLogLevel {
        fn from(value: &LogManagementLogCollectLogLogLevel) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64> for LogManagementLogCollectLogLogLevel {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogCollectLogLogLevel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The namespace associated with the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogCollectLogLogNamespace(String);
    impl std::ops::Deref for LogManagementLogCollectLogLogNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogCollectLogLogNamespace> for String {
        fn from(value: LogManagementLogCollectLogLogNamespace) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogCollectLogLogNamespace> for LogManagementLogCollectLogLogNamespace {
        fn from(value: &LogManagementLogCollectLogLogNamespace) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogCollectLogLogNamespace {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 64usize {
                return Err("longer than 64 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogCollectLogLogNamespace {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogCollectLogLogNamespace {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogCollectLogLogNamespace {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogCollectLogLogNamespace {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The template used for formatting the log message.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogCollectLogLogTemplate(String);
    impl std::ops::Deref for LogManagementLogCollectLogLogTemplate {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogCollectLogLogTemplate> for String {
        fn from(value: LogManagementLogCollectLogLogTemplate) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogCollectLogLogTemplate> for LogManagementLogCollectLogLogTemplate {
        fn from(value: &LogManagementLogCollectLogLogTemplate) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogCollectLogLogTemplate {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 4096usize {
                return Err("longer than 4096 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogCollectLogLogTemplate {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogCollectLogLogTemplate {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogCollectLogLogTemplate {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogCollectLogLogTemplate {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogCreateLog {
        ///The severity level of the log.
        pub level: LogManagementLogCreateLogLevel,
        ///The namespace associated with the log.
        pub namespace: LogManagementLogCreateLogNamespace,
        ///The template used for formatting the log message.
        pub template: LogManagementLogCreateLogTemplate,
    }
    impl From<&LogManagementLogCreateLog> for LogManagementLogCreateLog {
        fn from(value: &LogManagementLogCreateLog) -> Self {
            value.clone()
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogCreateLogEntry {
        pub attributes: serde_json::Map<String, serde_json::Value>,
        pub context: serde_json::Map<String, serde_json::Value>,
        ///The identity of the associated log.
        pub log_identity: uuid::Uuid,
        ///The timestamp indicating when the entry occurred.
        pub occurred_at: chrono::DateTime<chrono::offset::Utc>,
        ///The source of the entry, indicating what part of the system the entry is from.
        pub source: LogManagementLogCreateLogEntrySource,
        pub tags: Vec<String>,
    }
    impl From<&LogManagementLogCreateLogEntry> for LogManagementLogCreateLogEntry {
        fn from(value: &LogManagementLogCreateLogEntry) -> Self {
            value.clone()
        }
    }
    ///The source of the entry, indicating what part of the system the entry is from.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogCreateLogEntrySource(String);
    impl std::ops::Deref for LogManagementLogCreateLogEntrySource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogCreateLogEntrySource> for String {
        fn from(value: LogManagementLogCreateLogEntrySource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogCreateLogEntrySource> for LogManagementLogCreateLogEntrySource {
        fn from(value: &LogManagementLogCreateLogEntrySource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogCreateLogEntrySource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 128usize {
                return Err("longer than 128 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogCreateLogEntrySource {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogCreateLogEntrySource {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogCreateLogEntrySource {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogCreateLogEntrySource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The severity level of the log.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementLogCreateLogLevel(i64);
    impl std::ops::Deref for LogManagementLogCreateLogLevel {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementLogCreateLogLevel> for i64 {
        fn from(value: LogManagementLogCreateLogLevel) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogCreateLogLevel> for LogManagementLogCreateLogLevel {
        fn from(value: &LogManagementLogCreateLogLevel) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64> for LogManagementLogCreateLogLevel {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogCreateLogLevel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The namespace associated with the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogCreateLogNamespace(String);
    impl std::ops::Deref for LogManagementLogCreateLogNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogCreateLogNamespace> for String {
        fn from(value: LogManagementLogCreateLogNamespace) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogCreateLogNamespace> for LogManagementLogCreateLogNamespace {
        fn from(value: &LogManagementLogCreateLogNamespace) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogCreateLogNamespace {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 64usize {
                return Err("longer than 64 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogCreateLogNamespace {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogCreateLogNamespace {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogCreateLogNamespace {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogCreateLogNamespace {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The template used for formatting the log message.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogCreateLogTemplate(String);
    impl std::ops::Deref for LogManagementLogCreateLogTemplate {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogCreateLogTemplate> for String {
        fn from(value: LogManagementLogCreateLogTemplate) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogCreateLogTemplate> for LogManagementLogCreateLogTemplate {
        fn from(value: &LogManagementLogCreateLogTemplate) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogCreateLogTemplate {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 4096usize {
                return Err("longer than 4096 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogCreateLogTemplate {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogCreateLogTemplate {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogCreateLogTemplate {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogCreateLogTemplate {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The date and time after which the log entry was created.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogEntryFilterAfter(
        pub Option<chrono::DateTime<chrono::offset::Utc>>,
    );
    impl std::ops::Deref for LogManagementLogLogEntryFilterAfter {
        type Target = Option<chrono::DateTime<chrono::offset::Utc>>;
        fn deref(&self) -> &Option<chrono::DateTime<chrono::offset::Utc>> {
            &self.0
        }
    }
    impl From<LogManagementLogLogEntryFilterAfter> for Option<chrono::DateTime<chrono::offset::Utc>> {
        fn from(value: LogManagementLogLogEntryFilterAfter) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogEntryFilterAfter> for LogManagementLogLogEntryFilterAfter {
        fn from(value: &LogManagementLogLogEntryFilterAfter) -> Self {
            value.clone()
        }
    }
    impl From<Option<chrono::DateTime<chrono::offset::Utc>>> for LogManagementLogLogEntryFilterAfter {
        fn from(value: Option<chrono::DateTime<chrono::offset::Utc>>) -> Self {
            Self(value)
        }
    }
    ///The date and time before which the log entry was created.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogEntryFilterBefore(
        pub Option<chrono::DateTime<chrono::offset::Utc>>,
    );
    impl std::ops::Deref for LogManagementLogLogEntryFilterBefore {
        type Target = Option<chrono::DateTime<chrono::offset::Utc>>;
        fn deref(&self) -> &Option<chrono::DateTime<chrono::offset::Utc>> {
            &self.0
        }
    }
    impl From<LogManagementLogLogEntryFilterBefore> for Option<chrono::DateTime<chrono::offset::Utc>> {
        fn from(value: LogManagementLogLogEntryFilterBefore) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogEntryFilterBefore> for LogManagementLogLogEntryFilterBefore {
        fn from(value: &LogManagementLogLogEntryFilterBefore) -> Self {
            value.clone()
        }
    }
    impl From<Option<chrono::DateTime<chrono::offset::Utc>>> for LogManagementLogLogEntryFilterBefore {
        fn from(value: Option<chrono::DateTime<chrono::offset::Utc>>) -> Self {
            Self(value)
        }
    }
    ///The universally unique identifier (UUID) representing a log in the system.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogEntryFilterLogIdentity(pub Option<uuid::Uuid>);
    impl std::ops::Deref for LogManagementLogLogEntryFilterLogIdentity {
        type Target = Option<uuid::Uuid>;
        fn deref(&self) -> &Option<uuid::Uuid> {
            &self.0
        }
    }
    impl From<LogManagementLogLogEntryFilterLogIdentity> for Option<uuid::Uuid> {
        fn from(value: LogManagementLogLogEntryFilterLogIdentity) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogEntryFilterLogIdentity>
        for LogManagementLogLogEntryFilterLogIdentity
    {
        fn from(value: &LogManagementLogLogEntryFilterLogIdentity) -> Self {
            value.clone()
        }
    }
    impl From<Option<uuid::Uuid>> for LogManagementLogLogEntryFilterLogIdentity {
        fn from(value: Option<uuid::Uuid>) -> Self {
            Self(value)
        }
    }
    ///The order of the log entries.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogEntryFilterOrder {
        #[serde(rename = "ASC")]
        Asc,
        #[serde(rename = "DESC")]
        Desc,
    }
    impl From<&LogManagementLogLogEntryFilterOrder> for LogManagementLogLogEntryFilterOrder {
        fn from(value: &LogManagementLogLogEntryFilterOrder) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogEntryFilterOrder {
        fn to_string(&self) -> String {
            match *self {
                Self::Asc => "ASC".to_string(),
                Self::Desc => "DESC".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntryFilterOrder {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "ASC" => Ok(Self::Asc),
                "DESC" => Ok(Self::Desc),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogEntryFilterOrder {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogEntryFilterOrder {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogEntryFilterOrder {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl Default for LogManagementLogLogEntryFilterOrder {
        fn default() -> Self {
            LogManagementLogLogEntryFilterOrder::Desc
        }
    }
    ///The source of the log entry.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogEntryFilterSource(pub Option<String>);
    impl std::ops::Deref for LogManagementLogLogEntryFilterSource {
        type Target = Option<String>;
        fn deref(&self) -> &Option<String> {
            &self.0
        }
    }
    impl From<LogManagementLogLogEntryFilterSource> for Option<String> {
        fn from(value: LogManagementLogLogEntryFilterSource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogEntryFilterSource> for LogManagementLogLogEntryFilterSource {
        fn from(value: &LogManagementLogLogEntryFilterSource) -> Self {
            value.clone()
        }
    }
    impl From<Option<String>> for LogManagementLogLogEntryFilterSource {
        fn from(value: Option<String>) -> Self {
            Self(value)
        }
    }
    ///Resource representing a log entry.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogEntryResource {
        pub attributes: serde_json::Map<String, serde_json::Value>,
        pub context: serde_json::Map<String, serde_json::Value>,
        ///Timestamp when the log entry was created.
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        ///Unique identity of the log entry.
        pub identity: uuid::Uuid,
        ///Identity of the associated log.
        pub log_identity: uuid::Uuid,
        ///Timestamp when the log entry occurred.
        pub occurred_at: chrono::DateTime<chrono::offset::Utc>,
        ///Source of the log entry.
        pub source: LogManagementLogLogEntryResourceSource,
        ///Tags associated with the log entry.
        pub tags: Vec<LogManagementLogLogEntryResourceTagsItem>,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogEntryResourceType,
        ///Timestamp when the log entry was last updated.
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }
    impl From<&LogManagementLogLogEntryResource> for LogManagementLogLogEntryResource {
        fn from(value: &LogManagementLogLogEntryResource) -> Self {
            value.clone()
        }
    }
    ///Paginated collection of LogEntryResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogEntryResourcePaginatedCollectionResource {
        ///First item index.
        pub first: i64,
        ///Array of LogEntryResource.
        pub items: Vec<LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItem>,
        ///Number of items per page.
        pub items_per_page: i64,
        ///Last item index.
        pub last: i64,
        ///Next page number, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next: Option<i64>,
        ///Current page number.
        pub page: i64,
        ///Previous page number, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub previous: Option<i64>,
        ///Total number of items.
        pub total_items: i64,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogEntryResourcePaginatedCollectionResourceType,
    }
    impl From<&LogManagementLogLogEntryResourcePaginatedCollectionResource>
        for LogManagementLogLogEntryResourcePaginatedCollectionResource
    {
        fn from(value: &LogManagementLogLogEntryResourcePaginatedCollectionResource) -> Self {
            value.clone()
        }
    }
    ///Resource representing a log entry.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItem {
        pub attributes: serde_json::Map<String, serde_json::Value>,
        pub context: serde_json::Map<String, serde_json::Value>,
        ///Timestamp when the log entry was created.
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        ///Unique identity of the log entry.
        pub identity: uuid::Uuid,
        ///Identity of the associated log.
        pub log_identity: uuid::Uuid,
        ///Timestamp when the log entry occurred.
        pub occurred_at: chrono::DateTime<chrono::offset::Utc>,
        ///Source of the log entry.
        pub source: LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemSource,
        ///Tags associated with the log entry.
        pub tags: Vec<LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemTagsItem>,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemType,
        ///Timestamp when the log entry was last updated.
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }
    impl From<&LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItem>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItem
    {
        fn from(
            value: &LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItem,
        ) -> Self {
            value.clone()
        }
    }
    ///Source of the log entry.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemSource(String);
    impl std::ops::Deref
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemSource
    {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemSource> for String {
        fn from(
            value: LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemSource,
        ) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemSource>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemSource
    {
        fn from(
            value: &LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemSource,
        ) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemSource
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemSource
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemSource
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemSource
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemSource
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemTagsItem(String);
    impl std::ops::Deref
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemTagsItem
    {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemTagsItem> for String {
        fn from(
            value: LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemTagsItem,
        ) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemTagsItem>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemTagsItem
    {
        fn from(
            value: &LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemTagsItem,
        ) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemTagsItem
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemTagsItem
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemTagsItem
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemTagsItem
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemTagsItem
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemType {
        #[serde(rename = "log_entry")]
        LogEntry,
    }
    impl From<&LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemType>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemType
    {
        fn from(
            value: &LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntry => "log_entry".to_string(),
            }
        }
    }
    impl std::str::FromStr
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemType
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry" => Ok(Self::LogEntry),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogEntryResourcePaginatedCollectionResourceType {
        #[serde(rename = "paginated_collection")]
        PaginatedCollection,
    }
    impl From<&LogManagementLogLogEntryResourcePaginatedCollectionResourceType>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceType
    {
        fn from(value: &LogManagementLogLogEntryResourcePaginatedCollectionResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogEntryResourcePaginatedCollectionResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::PaginatedCollection => "paginated_collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntryResourcePaginatedCollectionResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "paginated_collection" => Ok(Self::PaginatedCollection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogEntryResourcePaginatedCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Source of the log entry.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogEntryResourceSource(String);
    impl std::ops::Deref for LogManagementLogLogEntryResourceSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogEntryResourceSource> for String {
        fn from(value: LogManagementLogLogEntryResourceSource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogEntryResourceSource> for LogManagementLogLogEntryResourceSource {
        fn from(value: &LogManagementLogLogEntryResourceSource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntryResourceSource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogEntryResourceSource {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogEntryResourceSource {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogEntryResourceSource {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogLogEntryResourceSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogEntryResourceTagsItem(String);
    impl std::ops::Deref for LogManagementLogLogEntryResourceTagsItem {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogEntryResourceTagsItem> for String {
        fn from(value: LogManagementLogLogEntryResourceTagsItem) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogEntryResourceTagsItem> for LogManagementLogLogEntryResourceTagsItem {
        fn from(value: &LogManagementLogLogEntryResourceTagsItem) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntryResourceTagsItem {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogEntryResourceTagsItem {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogEntryResourceTagsItem {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogEntryResourceTagsItem {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogLogEntryResourceTagsItem {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogEntryResourceType {
        #[serde(rename = "log_entry")]
        LogEntry,
    }
    impl From<&LogManagementLogLogEntryResourceType> for LogManagementLogLogEntryResourceType {
        fn from(value: &LogManagementLogLogEntryResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogEntryResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntry => "log_entry".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntryResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry" => Ok(Self::LogEntry),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogEntryResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogEntryResourceType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogEntryResourceType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Resource representing a log entry source.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogEntrySourceResource {
        ///Source of the log entry.
        pub source: LogManagementLogLogEntrySourceResourceSource,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogEntrySourceResourceType,
    }
    impl From<&LogManagementLogLogEntrySourceResource> for LogManagementLogLogEntrySourceResource {
        fn from(value: &LogManagementLogLogEntrySourceResource) -> Self {
            value.clone()
        }
    }
    ///Collection of LogEntrySourceResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogEntrySourceResourceCollectionResource {
        ///Array of LogEntrySourceResource.
        pub items: Vec<LogManagementLogLogEntrySourceResourceCollectionResourceItemsItem>,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogEntrySourceResourceCollectionResourceType,
    }
    impl From<&LogManagementLogLogEntrySourceResourceCollectionResource>
        for LogManagementLogLogEntrySourceResourceCollectionResource
    {
        fn from(value: &LogManagementLogLogEntrySourceResourceCollectionResource) -> Self {
            value.clone()
        }
    }
    ///Resource representing a log entry source.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogEntrySourceResourceCollectionResourceItemsItem {
        ///Source of the log entry.
        pub source: LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemSource,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemType,
    }
    impl From<&LogManagementLogLogEntrySourceResourceCollectionResourceItemsItem>
        for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItem
    {
        fn from(value: &LogManagementLogLogEntrySourceResourceCollectionResourceItemsItem) -> Self {
            value.clone()
        }
    }
    ///Source of the log entry.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemSource(String);
    impl std::ops::Deref for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemSource> for String {
        fn from(
            value: LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemSource,
        ) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemSource>
        for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemSource
    {
        fn from(
            value: &LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemSource,
        ) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemSource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemSource
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemSource
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemSource
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemSource
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemType {
        #[serde(rename = "log_entry_source")]
        LogEntrySource,
    }
    impl From<&LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemType>
        for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemType
    {
        fn from(
            value: &LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntrySource => "log_entry_source".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_source" => Ok(Self::LogEntrySource),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogEntrySourceResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogEntrySourceResourceCollectionResourceType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementLogLogEntrySourceResourceCollectionResourceType>
        for LogManagementLogLogEntrySourceResourceCollectionResourceType
    {
        fn from(value: &LogManagementLogLogEntrySourceResourceCollectionResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogEntrySourceResourceCollectionResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntrySourceResourceCollectionResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogEntrySourceResourceCollectionResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogEntrySourceResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogEntrySourceResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Source of the log entry.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogEntrySourceResourceSource(String);
    impl std::ops::Deref for LogManagementLogLogEntrySourceResourceSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogEntrySourceResourceSource> for String {
        fn from(value: LogManagementLogLogEntrySourceResourceSource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogEntrySourceResourceSource>
        for LogManagementLogLogEntrySourceResourceSource
    {
        fn from(value: &LogManagementLogLogEntrySourceResourceSource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntrySourceResourceSource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogEntrySourceResourceSource {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogEntrySourceResourceSource {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogEntrySourceResourceSource {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogLogEntrySourceResourceSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogEntrySourceResourceType {
        #[serde(rename = "log_entry_source")]
        LogEntrySource,
    }
    impl From<&LogManagementLogLogEntrySourceResourceType>
        for LogManagementLogLogEntrySourceResourceType
    {
        fn from(value: &LogManagementLogLogEntrySourceResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogEntrySourceResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntrySource => "log_entry_source".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntrySourceResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_source" => Ok(Self::LogEntrySource),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogEntrySourceResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogEntrySourceResourceType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogEntrySourceResourceType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Resource representing a log entry tag.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogEntryTagResource {
        ///Tag of the log entry.
        pub tag: LogManagementLogLogEntryTagResourceTag,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogEntryTagResourceType,
    }
    impl From<&LogManagementLogLogEntryTagResource> for LogManagementLogLogEntryTagResource {
        fn from(value: &LogManagementLogLogEntryTagResource) -> Self {
            value.clone()
        }
    }
    ///Collection of LogEntryTagResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogEntryTagResourceCollectionResource {
        ///Array of LogEntryTagResource.
        pub items: Vec<LogManagementLogLogEntryTagResourceCollectionResourceItemsItem>,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogEntryTagResourceCollectionResourceType,
    }
    impl From<&LogManagementLogLogEntryTagResourceCollectionResource>
        for LogManagementLogLogEntryTagResourceCollectionResource
    {
        fn from(value: &LogManagementLogLogEntryTagResourceCollectionResource) -> Self {
            value.clone()
        }
    }
    ///Resource representing a log entry tag.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogEntryTagResourceCollectionResourceItemsItem {
        ///Tag of the log entry.
        pub tag: LogManagementLogLogEntryTagResourceCollectionResourceItemsItemTag,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogEntryTagResourceCollectionResourceItemsItemType,
    }
    impl From<&LogManagementLogLogEntryTagResourceCollectionResourceItemsItem>
        for LogManagementLogLogEntryTagResourceCollectionResourceItemsItem
    {
        fn from(value: &LogManagementLogLogEntryTagResourceCollectionResourceItemsItem) -> Self {
            value.clone()
        }
    }
    ///Tag of the log entry.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogEntryTagResourceCollectionResourceItemsItemTag(String);
    impl std::ops::Deref for LogManagementLogLogEntryTagResourceCollectionResourceItemsItemTag {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogEntryTagResourceCollectionResourceItemsItemTag> for String {
        fn from(value: LogManagementLogLogEntryTagResourceCollectionResourceItemsItemTag) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogEntryTagResourceCollectionResourceItemsItemTag>
        for LogManagementLogLogEntryTagResourceCollectionResourceItemsItemTag
    {
        fn from(value: &LogManagementLogLogEntryTagResourceCollectionResourceItemsItemTag) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntryTagResourceCollectionResourceItemsItemTag {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogEntryTagResourceCollectionResourceItemsItemTag
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogEntryTagResourceCollectionResourceItemsItemTag
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogEntryTagResourceCollectionResourceItemsItemTag
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementLogLogEntryTagResourceCollectionResourceItemsItemTag
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogEntryTagResourceCollectionResourceItemsItemType {
        #[serde(rename = "log_entry_tag")]
        LogEntryTag,
    }
    impl From<&LogManagementLogLogEntryTagResourceCollectionResourceItemsItemType>
        for LogManagementLogLogEntryTagResourceCollectionResourceItemsItemType
    {
        fn from(
            value: &LogManagementLogLogEntryTagResourceCollectionResourceItemsItemType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogEntryTagResourceCollectionResourceItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntryTag => "log_entry_tag".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntryTagResourceCollectionResourceItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_tag" => Ok(Self::LogEntryTag),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogEntryTagResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogEntryTagResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogEntryTagResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogEntryTagResourceCollectionResourceType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementLogLogEntryTagResourceCollectionResourceType>
        for LogManagementLogLogEntryTagResourceCollectionResourceType
    {
        fn from(value: &LogManagementLogLogEntryTagResourceCollectionResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogEntryTagResourceCollectionResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntryTagResourceCollectionResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogEntryTagResourceCollectionResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogEntryTagResourceCollectionResourceType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogEntryTagResourceCollectionResourceType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Tag of the log entry.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogEntryTagResourceTag(String);
    impl std::ops::Deref for LogManagementLogLogEntryTagResourceTag {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogEntryTagResourceTag> for String {
        fn from(value: LogManagementLogLogEntryTagResourceTag) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogEntryTagResourceTag> for LogManagementLogLogEntryTagResourceTag {
        fn from(value: &LogManagementLogLogEntryTagResourceTag) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntryTagResourceTag {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogEntryTagResourceTag {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogEntryTagResourceTag {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogEntryTagResourceTag {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogLogEntryTagResourceTag {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogEntryTagResourceType {
        #[serde(rename = "log_entry_tag")]
        LogEntryTag,
    }
    impl From<&LogManagementLogLogEntryTagResourceType> for LogManagementLogLogEntryTagResourceType {
        fn from(value: &LogManagementLogLogEntryTagResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogEntryTagResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntryTag => "log_entry_tag".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogEntryTagResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_tag" => Ok(Self::LogEntryTag),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogEntryTagResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogEntryTagResourceType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogEntryTagResourceType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///The string that the log template must contain.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogFilterContains(pub Option<String>);
    impl std::ops::Deref for LogManagementLogLogFilterContains {
        type Target = Option<String>;
        fn deref(&self) -> &Option<String> {
            &self.0
        }
    }
    impl From<LogManagementLogLogFilterContains> for Option<String> {
        fn from(value: LogManagementLogLogFilterContains) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogFilterContains> for LogManagementLogLogFilterContains {
        fn from(value: &LogManagementLogLogFilterContains) -> Self {
            value.clone()
        }
    }
    impl From<Option<String>> for LogManagementLogLogFilterContains {
        fn from(value: Option<String>) -> Self {
            Self(value)
        }
    }
    ///The date and time after which the log occurred.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogFilterFrom(pub Option<chrono::DateTime<chrono::offset::Utc>>);
    impl std::ops::Deref for LogManagementLogLogFilterFrom {
        type Target = Option<chrono::DateTime<chrono::offset::Utc>>;
        fn deref(&self) -> &Option<chrono::DateTime<chrono::offset::Utc>> {
            &self.0
        }
    }
    impl From<LogManagementLogLogFilterFrom> for Option<chrono::DateTime<chrono::offset::Utc>> {
        fn from(value: LogManagementLogLogFilterFrom) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogFilterFrom> for LogManagementLogLogFilterFrom {
        fn from(value: &LogManagementLogLogFilterFrom) -> Self {
            value.clone()
        }
    }
    impl From<Option<chrono::DateTime<chrono::offset::Utc>>> for LogManagementLogLogFilterFrom {
        fn from(value: Option<chrono::DateTime<chrono::offset::Utc>>) -> Self {
            Self(value)
        }
    }
    ///The levels of the log entries.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogFilterLevels(pub Vec<LogManagementLogLogFilterLevelsItem>);
    impl std::ops::Deref for LogManagementLogLogFilterLevels {
        type Target = Vec<LogManagementLogLogFilterLevelsItem>;
        fn deref(&self) -> &Vec<LogManagementLogLogFilterLevelsItem> {
            &self.0
        }
    }
    impl From<LogManagementLogLogFilterLevels> for Vec<LogManagementLogLogFilterLevelsItem> {
        fn from(value: LogManagementLogLogFilterLevels) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogFilterLevels> for LogManagementLogLogFilterLevels {
        fn from(value: &LogManagementLogLogFilterLevels) -> Self {
            value.clone()
        }
    }
    impl From<Vec<LogManagementLogLogFilterLevelsItem>> for LogManagementLogLogFilterLevels {
        fn from(value: Vec<LogManagementLogLogFilterLevelsItem>) -> Self {
            Self(value)
        }
    }
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementLogLogFilterLevelsItem(i64);
    impl std::ops::Deref for LogManagementLogLogFilterLevelsItem {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementLogLogFilterLevelsItem> for i64 {
        fn from(value: LogManagementLogLogFilterLevelsItem) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogFilterLevelsItem> for LogManagementLogLogFilterLevelsItem {
        fn from(value: &LogManagementLogLogFilterLevelsItem) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64> for LogManagementLogLogFilterLevelsItem {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogLogFilterLevelsItem {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The order of the logs.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogFilterOrder {
        #[serde(rename = "ASC")]
        Asc,
        #[serde(rename = "DESC")]
        Desc,
    }
    impl From<&LogManagementLogLogFilterOrder> for LogManagementLogLogFilterOrder {
        fn from(value: &LogManagementLogLogFilterOrder) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogFilterOrder {
        fn to_string(&self) -> String {
            match *self {
                Self::Asc => "ASC".to_string(),
                Self::Desc => "DESC".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogFilterOrder {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "ASC" => Ok(Self::Asc),
                "DESC" => Ok(Self::Desc),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogFilterOrder {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogFilterOrder {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogFilterOrder {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl Default for LogManagementLogLogFilterOrder {
        fn default() -> Self {
            LogManagementLogLogFilterOrder::Desc
        }
    }
    ///The field to sort the logs by.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogFilterSortBy {
        #[serde(rename = "createdAt")]
        CreatedAt,
        #[serde(rename = "updatedAt")]
        UpdatedAt,
        #[serde(rename = "firstEntryOccurredAt")]
        FirstEntryOccurredAt,
        #[serde(rename = "lastEntryOccurredAt")]
        LastEntryOccurredAt,
        #[serde(rename = "level")]
        Level,
    }
    impl From<&LogManagementLogLogFilterSortBy> for LogManagementLogLogFilterSortBy {
        fn from(value: &LogManagementLogLogFilterSortBy) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogFilterSortBy {
        fn to_string(&self) -> String {
            match *self {
                Self::CreatedAt => "createdAt".to_string(),
                Self::UpdatedAt => "updatedAt".to_string(),
                Self::FirstEntryOccurredAt => "firstEntryOccurredAt".to_string(),
                Self::LastEntryOccurredAt => "lastEntryOccurredAt".to_string(),
                Self::Level => "level".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogFilterSortBy {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "createdAt" => Ok(Self::CreatedAt),
                "updatedAt" => Ok(Self::UpdatedAt),
                "firstEntryOccurredAt" => Ok(Self::FirstEntryOccurredAt),
                "lastEntryOccurredAt" => Ok(Self::LastEntryOccurredAt),
                "level" => Ok(Self::Level),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogFilterSortBy {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogFilterSortBy {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogFilterSortBy {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl Default for LogManagementLogLogFilterSortBy {
        fn default() -> Self {
            LogManagementLogLogFilterSortBy::LastEntryOccurredAt
        }
    }
    ///The date and time before which the log occurred.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogFilterTo(pub Option<chrono::DateTime<chrono::offset::Utc>>);
    impl std::ops::Deref for LogManagementLogLogFilterTo {
        type Target = Option<chrono::DateTime<chrono::offset::Utc>>;
        fn deref(&self) -> &Option<chrono::DateTime<chrono::offset::Utc>> {
            &self.0
        }
    }
    impl From<LogManagementLogLogFilterTo> for Option<chrono::DateTime<chrono::offset::Utc>> {
        fn from(value: LogManagementLogLogFilterTo) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogFilterTo> for LogManagementLogLogFilterTo {
        fn from(value: &LogManagementLogLogFilterTo) -> Self {
            value.clone()
        }
    }
    impl From<Option<chrono::DateTime<chrono::offset::Utc>>> for LogManagementLogLogFilterTo {
        fn from(value: Option<chrono::DateTime<chrono::offset::Utc>>) -> Self {
            Self(value)
        }
    }
    ///Resource representing a log namespace.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogNamespaceResource {
        ///Namespace of the log.
        pub namespace: LogManagementLogLogNamespaceResourceNamespace,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogNamespaceResourceType,
    }
    impl From<&LogManagementLogLogNamespaceResource> for LogManagementLogLogNamespaceResource {
        fn from(value: &LogManagementLogLogNamespaceResource) -> Self {
            value.clone()
        }
    }
    ///Collection of LogNamespaceResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogNamespaceResourceCollectionResource {
        ///Array of LogNamespaceResource.
        pub items: Vec<LogManagementLogLogNamespaceResourceCollectionResourceItemsItem>,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogNamespaceResourceCollectionResourceType,
    }
    impl From<&LogManagementLogLogNamespaceResourceCollectionResource>
        for LogManagementLogLogNamespaceResourceCollectionResource
    {
        fn from(value: &LogManagementLogLogNamespaceResourceCollectionResource) -> Self {
            value.clone()
        }
    }
    ///Resource representing a log namespace.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogNamespaceResourceCollectionResourceItemsItem {
        ///Namespace of the log.
        pub namespace: LogManagementLogLogNamespaceResourceCollectionResourceItemsItemNamespace,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogNamespaceResourceCollectionResourceItemsItemType,
    }
    impl From<&LogManagementLogLogNamespaceResourceCollectionResourceItemsItem>
        for LogManagementLogLogNamespaceResourceCollectionResourceItemsItem
    {
        fn from(value: &LogManagementLogLogNamespaceResourceCollectionResourceItemsItem) -> Self {
            value.clone()
        }
    }
    ///Namespace of the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogNamespaceResourceCollectionResourceItemsItemNamespace(String);
    impl std::ops::Deref for LogManagementLogLogNamespaceResourceCollectionResourceItemsItemNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogNamespaceResourceCollectionResourceItemsItemNamespace> for String {
        fn from(
            value: LogManagementLogLogNamespaceResourceCollectionResourceItemsItemNamespace,
        ) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogNamespaceResourceCollectionResourceItemsItemNamespace>
        for LogManagementLogLogNamespaceResourceCollectionResourceItemsItemNamespace
    {
        fn from(
            value: &LogManagementLogLogNamespaceResourceCollectionResourceItemsItemNamespace,
        ) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr
        for LogManagementLogLogNamespaceResourceCollectionResourceItemsItemNamespace
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogNamespaceResourceCollectionResourceItemsItemNamespace
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogNamespaceResourceCollectionResourceItemsItemNamespace
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogNamespaceResourceCollectionResourceItemsItemNamespace
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementLogLogNamespaceResourceCollectionResourceItemsItemNamespace
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogNamespaceResourceCollectionResourceItemsItemType {
        #[serde(rename = "log_namespace")]
        LogNamespace,
    }
    impl From<&LogManagementLogLogNamespaceResourceCollectionResourceItemsItemType>
        for LogManagementLogLogNamespaceResourceCollectionResourceItemsItemType
    {
        fn from(
            value: &LogManagementLogLogNamespaceResourceCollectionResourceItemsItemType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogNamespaceResourceCollectionResourceItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogNamespace => "log_namespace".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogNamespaceResourceCollectionResourceItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_namespace" => Ok(Self::LogNamespace),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogNamespaceResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogNamespaceResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogNamespaceResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogNamespaceResourceCollectionResourceType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementLogLogNamespaceResourceCollectionResourceType>
        for LogManagementLogLogNamespaceResourceCollectionResourceType
    {
        fn from(value: &LogManagementLogLogNamespaceResourceCollectionResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogNamespaceResourceCollectionResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogNamespaceResourceCollectionResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogNamespaceResourceCollectionResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogNamespaceResourceCollectionResourceType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogNamespaceResourceCollectionResourceType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Namespace of the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogNamespaceResourceNamespace(String);
    impl std::ops::Deref for LogManagementLogLogNamespaceResourceNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogNamespaceResourceNamespace> for String {
        fn from(value: LogManagementLogLogNamespaceResourceNamespace) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogNamespaceResourceNamespace>
        for LogManagementLogLogNamespaceResourceNamespace
    {
        fn from(value: &LogManagementLogLogNamespaceResourceNamespace) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogLogNamespaceResourceNamespace {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogNamespaceResourceNamespace {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogNamespaceResourceNamespace {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogNamespaceResourceNamespace {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogLogNamespaceResourceNamespace {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogNamespaceResourceType {
        #[serde(rename = "log_namespace")]
        LogNamespace,
    }
    impl From<&LogManagementLogLogNamespaceResourceType> for LogManagementLogLogNamespaceResourceType {
        fn from(value: &LogManagementLogLogNamespaceResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogNamespaceResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogNamespace => "log_namespace".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogNamespaceResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_namespace" => Ok(Self::LogNamespace),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogNamespaceResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogNamespaceResourceType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogNamespaceResourceType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Resource representing a log.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogResource {
        ///Timestamp when the log was created.
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        ///Timestamp of the first entry occurrence, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub first_entry_occurred_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        ///Unique identity of the log.
        pub identity: uuid::Uuid,
        ///Timestamp of the last entry occurrence, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub last_entry_occurred_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub level: LogManagementLogLogResourceLevel,
        ///The namespace of the log.
        pub namespace: LogManagementLogLogResourceNamespace,
        ///The template for formatting the message.
        pub template: LogManagementLogLogResourceTemplate,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogResourceType,
        ///Timestamp when the log was last updated.
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }
    impl From<&LogManagementLogLogResource> for LogManagementLogLogResource {
        fn from(value: &LogManagementLogLogResource) -> Self {
            value.clone()
        }
    }
    ///The log level.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogResourceLevel {
        ///Name of the log level.
        pub name: LogManagementLogLogResourceLevelName,
        ///Value of the log level.
        pub value: LogManagementLogLogResourceLevelValue,
    }
    impl From<&LogManagementLogLogResourceLevel> for LogManagementLogLogResourceLevel {
        fn from(value: &LogManagementLogLogResourceLevel) -> Self {
            value.clone()
        }
    }
    ///Name of the log level.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogResourceLevelName {
        Debug,
        Info,
        Notice,
        Warning,
        Error,
        Critical,
        Alert,
        Emergency,
    }
    impl From<&LogManagementLogLogResourceLevelName> for LogManagementLogLogResourceLevelName {
        fn from(value: &LogManagementLogLogResourceLevelName) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogResourceLevelName {
        fn to_string(&self) -> String {
            match *self {
                Self::Debug => "Debug".to_string(),
                Self::Info => "Info".to_string(),
                Self::Notice => "Notice".to_string(),
                Self::Warning => "Warning".to_string(),
                Self::Error => "Error".to_string(),
                Self::Critical => "Critical".to_string(),
                Self::Alert => "Alert".to_string(),
                Self::Emergency => "Emergency".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogResourceLevelName {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "Debug" => Ok(Self::Debug),
                "Info" => Ok(Self::Info),
                "Notice" => Ok(Self::Notice),
                "Warning" => Ok(Self::Warning),
                "Error" => Ok(Self::Error),
                "Critical" => Ok(Self::Critical),
                "Alert" => Ok(Self::Alert),
                "Emergency" => Ok(Self::Emergency),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogResourceLevelName {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogResourceLevelName {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogResourceLevelName {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Value of the log level.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementLogLogResourceLevelValue(i64);
    impl std::ops::Deref for LogManagementLogLogResourceLevelValue {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementLogLogResourceLevelValue> for i64 {
        fn from(value: LogManagementLogLogResourceLevelValue) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogResourceLevelValue> for LogManagementLogLogResourceLevelValue {
        fn from(value: &LogManagementLogLogResourceLevelValue) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64> for LogManagementLogLogResourceLevelValue {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogLogResourceLevelValue {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The namespace of the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogResourceNamespace(String);
    impl std::ops::Deref for LogManagementLogLogResourceNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogResourceNamespace> for String {
        fn from(value: LogManagementLogLogResourceNamespace) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogResourceNamespace> for LogManagementLogLogResourceNamespace {
        fn from(value: &LogManagementLogLogResourceNamespace) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogLogResourceNamespace {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogResourceNamespace {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogResourceNamespace {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogResourceNamespace {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogLogResourceNamespace {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Paginated collection of LogResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogResourcePaginatedCollectionResource {
        ///First item index.
        pub first: i64,
        ///Array of LogResource.
        pub items: Vec<LogManagementLogLogResourcePaginatedCollectionResourceItemsItem>,
        ///Number of items per page.
        pub items_per_page: i64,
        ///Last item index.
        pub last: i64,
        ///Next page number, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next: Option<i64>,
        ///Current page number.
        pub page: i64,
        ///Previous page number, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub previous: Option<i64>,
        ///Total number of items.
        pub total_items: i64,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogResourcePaginatedCollectionResourceType,
    }
    impl From<&LogManagementLogLogResourcePaginatedCollectionResource>
        for LogManagementLogLogResourcePaginatedCollectionResource
    {
        fn from(value: &LogManagementLogLogResourcePaginatedCollectionResource) -> Self {
            value.clone()
        }
    }
    ///Resource representing a log.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogResourcePaginatedCollectionResourceItemsItem {
        ///Timestamp when the log was created.
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        ///Timestamp of the first entry occurrence, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub first_entry_occurred_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        ///Unique identity of the log.
        pub identity: uuid::Uuid,
        ///Timestamp of the last entry occurrence, if available.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub last_entry_occurred_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub level: LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevel,
        ///The namespace of the log.
        pub namespace: LogManagementLogLogResourcePaginatedCollectionResourceItemsItemNamespace,
        ///The template for formatting the message.
        pub template: LogManagementLogLogResourcePaginatedCollectionResourceItemsItemTemplate,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogLogResourcePaginatedCollectionResourceItemsItemType,
        ///Timestamp when the log was last updated.
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }
    impl From<&LogManagementLogLogResourcePaginatedCollectionResourceItemsItem>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItem
    {
        fn from(value: &LogManagementLogLogResourcePaginatedCollectionResourceItemsItem) -> Self {
            value.clone()
        }
    }
    ///The log level.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevel {
        ///Name of the log level.
        pub name: LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelName,
        ///Value of the log level.
        pub value: LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelValue,
    }
    impl From<&LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevel>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevel
    {
        fn from(
            value: &LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevel,
        ) -> Self {
            value.clone()
        }
    }
    ///Name of the log level.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelName {
        Debug,
        Info,
        Notice,
        Warning,
        Error,
        Critical,
        Alert,
        Emergency,
    }
    impl From<&LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelName>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelName
    {
        fn from(
            value: &LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelName,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelName {
        fn to_string(&self) -> String {
            match *self {
                Self::Debug => "Debug".to_string(),
                Self::Info => "Info".to_string(),
                Self::Notice => "Notice".to_string(),
                Self::Warning => "Warning".to_string(),
                Self::Error => "Error".to_string(),
                Self::Critical => "Critical".to_string(),
                Self::Alert => "Alert".to_string(),
                Self::Emergency => "Emergency".to_string(),
            }
        }
    }
    impl std::str::FromStr
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelName
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "Debug" => Ok(Self::Debug),
                "Info" => Ok(Self::Info),
                "Notice" => Ok(Self::Notice),
                "Warning" => Ok(Self::Warning),
                "Error" => Ok(Self::Error),
                "Critical" => Ok(Self::Critical),
                "Alert" => Ok(Self::Alert),
                "Emergency" => Ok(Self::Emergency),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelName
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelName
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelName
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Value of the log level.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelValue(i64);
    impl std::ops::Deref for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelValue {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelValue> for i64 {
        fn from(
            value: LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelValue,
        ) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelValue>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelValue
    {
        fn from(
            value: &LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelValue,
        ) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelValue
    {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemLevelValue
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The namespace of the log.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogResourcePaginatedCollectionResourceItemsItemNamespace(String);
    impl std::ops::Deref for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemNamespace {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogResourcePaginatedCollectionResourceItemsItemNamespace> for String {
        fn from(
            value: LogManagementLogLogResourcePaginatedCollectionResourceItemsItemNamespace,
        ) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogResourcePaginatedCollectionResourceItemsItemNamespace>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemNamespace
    {
        fn from(
            value: &LogManagementLogLogResourcePaginatedCollectionResourceItemsItemNamespace,
        ) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemNamespace
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemNamespace
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemNamespace
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemNamespace
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemNamespace
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///The template for formatting the message.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogResourcePaginatedCollectionResourceItemsItemTemplate(String);
    impl std::ops::Deref for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemTemplate {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogResourcePaginatedCollectionResourceItemsItemTemplate> for String {
        fn from(
            value: LogManagementLogLogResourcePaginatedCollectionResourceItemsItemTemplate,
        ) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogResourcePaginatedCollectionResourceItemsItemTemplate>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemTemplate
    {
        fn from(
            value: &LogManagementLogLogResourcePaginatedCollectionResourceItemsItemTemplate,
        ) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemTemplate {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemTemplate
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemTemplate
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemTemplate
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemTemplate
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogResourcePaginatedCollectionResourceItemsItemType {
        #[serde(rename = "log")]
        Log,
    }
    impl From<&LogManagementLogLogResourcePaginatedCollectionResourceItemsItemType>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemType
    {
        fn from(
            value: &LogManagementLogLogResourcePaginatedCollectionResourceItemsItemType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemType {
        fn to_string(&self) -> String {
            match *self {
                Self::Log => "log".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log" => Ok(Self::Log),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogLogResourcePaginatedCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogResourcePaginatedCollectionResourceType {
        #[serde(rename = "paginated_collection")]
        PaginatedCollection,
    }
    impl From<&LogManagementLogLogResourcePaginatedCollectionResourceType>
        for LogManagementLogLogResourcePaginatedCollectionResourceType
    {
        fn from(value: &LogManagementLogLogResourcePaginatedCollectionResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogResourcePaginatedCollectionResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::PaginatedCollection => "paginated_collection".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogResourcePaginatedCollectionResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "paginated_collection" => Ok(Self::PaginatedCollection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogResourcePaginatedCollectionResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogResourcePaginatedCollectionResourceType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogResourcePaginatedCollectionResourceType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///The template for formatting the message.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogLogResourceTemplate(String);
    impl std::ops::Deref for LogManagementLogLogResourceTemplate {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogLogResourceTemplate> for String {
        fn from(value: LogManagementLogLogResourceTemplate) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogLogResourceTemplate> for LogManagementLogLogResourceTemplate {
        fn from(value: &LogManagementLogLogResourceTemplate) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogLogResourceTemplate {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogResourceTemplate {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogResourceTemplate {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogResourceTemplate {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogLogResourceTemplate {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogLogResourceType {
        #[serde(rename = "log")]
        Log,
    }
    impl From<&LogManagementLogLogResourceType> for LogManagementLogLogResourceType {
        fn from(value: &LogManagementLogLogResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogLogResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::Log => "log".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogLogResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log" => Ok(Self::Log),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogLogResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogLogResourceType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogLogResourceType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///The frequency for log statistics, specifying how the data should be aggregated.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsFrequency {
        #[serde(rename = "yearly")]
        Yearly,
        #[serde(rename = "quarterly")]
        Quarterly,
        #[serde(rename = "monthly")]
        Monthly,
        #[serde(rename = "weekly")]
        Weekly,
        #[serde(rename = "daily")]
        Daily,
        #[serde(rename = "hourly")]
        Hourly,
    }
    impl From<&LogManagementLogStatisticsFrequency> for LogManagementLogStatisticsFrequency {
        fn from(value: &LogManagementLogStatisticsFrequency) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogStatisticsFrequency {
        fn to_string(&self) -> String {
            match *self {
                Self::Yearly => "yearly".to_string(),
                Self::Quarterly => "quarterly".to_string(),
                Self::Monthly => "monthly".to_string(),
                Self::Weekly => "weekly".to_string(),
                Self::Daily => "daily".to_string(),
                Self::Hourly => "hourly".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogStatisticsFrequency {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "yearly" => Ok(Self::Yearly),
                "quarterly" => Ok(Self::Quarterly),
                "monthly" => Ok(Self::Monthly),
                "weekly" => Ok(Self::Weekly),
                "daily" => Ok(Self::Daily),
                "hourly" => Ok(Self::Hourly),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogStatisticsFrequency {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogStatisticsFrequency {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogStatisticsFrequency {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Resource representing the frequency count of log entries.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogEntryFrequencyCountResource {
        ///Count of log entries for the specified date.
        pub count: std::num::NonZeroU64,
        ///Date associated with the frequency count.
        pub date: chrono::DateTime<chrono::offset::Utc>,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogStatisticsLogEntryFrequencyCountResourceType,
    }
    impl From<&LogManagementLogStatisticsLogEntryFrequencyCountResource>
        for LogManagementLogStatisticsLogEntryFrequencyCountResource
    {
        fn from(value: &LogManagementLogStatisticsLogEntryFrequencyCountResource) -> Self {
            value.clone()
        }
    }
    ///Collection of LogEntryFrequencyCountResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResource {
        ///Array of LogEntryFrequencyCountResource.
        pub items: Vec<
            LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItem,
        >,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceType,
    }
    impl From<&LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResource>
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResource
    {
        fn from(
            value: &LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResource,
        ) -> Self {
            value.clone()
        }
    }
    ///Resource representing the frequency count of log entries.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItem {
        ///Count of log entries for the specified date.
        pub count: std::num::NonZeroU64,
        ///Date associated with the frequency count.
        pub date: chrono::DateTime<chrono::offset::Utc>,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_:
            LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItemType,
    }
    impl From<&LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItem>
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItem
    {
        fn from(
            value: &LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItem,
        ) -> Self {
            value.clone()
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItemType {
        #[serde(rename = "log_entry_frequency_count")]
        LogEntryFrequencyCount,
    }
    impl From<
        &LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItemType,
    >
    for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItemType {
        fn from(
            value: &LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItemType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItemType
    {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntryFrequencyCount => "log_entry_frequency_count".to_string(),
            }
        }
    }
    impl std::str::FromStr
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItemType
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_frequency_count" => Ok(Self::LogEntryFrequencyCount),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceType>
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceType
    {
        fn from(
            value: &LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceType
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogEntryFrequencyCountResourceType {
        #[serde(rename = "log_entry_frequency_count")]
        LogEntryFrequencyCount,
    }
    impl From<&LogManagementLogStatisticsLogEntryFrequencyCountResourceType>
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceType
    {
        fn from(value: &LogManagementLogStatisticsLogEntryFrequencyCountResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogStatisticsLogEntryFrequencyCountResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntryFrequencyCount => "log_entry_frequency_count".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogStatisticsLogEntryFrequencyCountResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_frequency_count" => Ok(Self::LogEntryFrequencyCount),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogStatisticsLogEntryFrequencyCountResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogEntryFrequencyCountResourceType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Resource representing the frequency count of log entries by source.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogEntrySourceFrequencyResource {
        ///Count of log entries for the specified source.
        pub count: std::num::NonZeroU64,
        pub percentage: f64,
        ///Source associated with the frequency count.
        pub source: LogManagementLogStatisticsLogEntrySourceFrequencyResourceSource,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogStatisticsLogEntrySourceFrequencyResourceType,
    }
    impl From<&LogManagementLogStatisticsLogEntrySourceFrequencyResource>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResource
    {
        fn from(value: &LogManagementLogStatisticsLogEntrySourceFrequencyResource) -> Self {
            value.clone()
        }
    }
    ///Collection of LogEntrySourceFrequencyResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResource {
        ///Array of LogEntrySourceFrequencyResource.
        pub items: Vec<
            LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItem,
        >,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceType,
    }
    impl From<&LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResource>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResource
    {
        fn from(
            value: &LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResource,
        ) -> Self {
            value.clone()
        }
    }
    ///Resource representing the frequency count of log entries by source.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItem {
        ///Count of log entries for the specified source.
        pub count: std::num::NonZeroU64,
        pub percentage: f64,
        ///Source associated with the frequency count.
        pub source: LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemSource,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemType,
    }
    impl From<&LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItem>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItem
    {
        fn from(
            value: &LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItem,
        ) -> Self {
            value.clone()
        }
    }
    ///Source associated with the frequency count.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemSource(
        String,
    );
    impl std::ops::Deref
    for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<
        LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemSource,
    > for String {
        fn from(
            value: LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemSource,
        ) -> Self {
            value.0
        }
    }
    impl From<
        &LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemSource,
    >
    for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemSource {
        fn from(
            value: &LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemSource,
        ) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr
    for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemSource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 128usize {
                return Err("longer than 128 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
    for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemSource {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
    for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemSource {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
    for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemSource {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
    for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemType {
        #[serde(rename = "log_entry_source_frequency")]
        LogEntrySourceFrequency,
    }
    impl From<
        &LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemType,
    >
    for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemType {
        fn from(
            value: &LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemType
    {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntrySourceFrequency => "log_entry_source_frequency".to_string(),
            }
        }
    }
    impl std::str::FromStr
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemType
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_source_frequency" => Ok(Self::LogEntrySourceFrequency),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceType>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceType
    {
        fn from(
            value: &LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceType
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Source associated with the frequency count.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogStatisticsLogEntrySourceFrequencyResourceSource(String);
    impl std::ops::Deref for LogManagementLogStatisticsLogEntrySourceFrequencyResourceSource {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogStatisticsLogEntrySourceFrequencyResourceSource> for String {
        fn from(value: LogManagementLogStatisticsLogEntrySourceFrequencyResourceSource) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogStatisticsLogEntrySourceFrequencyResourceSource>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceSource
    {
        fn from(value: &LogManagementLogStatisticsLogEntrySourceFrequencyResourceSource) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogStatisticsLogEntrySourceFrequencyResourceSource {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() > 128usize {
                return Err("longer than 128 characters");
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceSource
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceSource
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceSource
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceSource
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogEntrySourceFrequencyResourceType {
        #[serde(rename = "log_entry_source_frequency")]
        LogEntrySourceFrequency,
    }
    impl From<&LogManagementLogStatisticsLogEntrySourceFrequencyResourceType>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceType
    {
        fn from(value: &LogManagementLogStatisticsLogEntrySourceFrequencyResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogStatisticsLogEntrySourceFrequencyResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntrySourceFrequency => "log_entry_source_frequency".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogStatisticsLogEntrySourceFrequencyResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_source_frequency" => Ok(Self::LogEntrySourceFrequency),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogStatisticsLogEntrySourceFrequencyResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogEntrySourceFrequencyResourceType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Resource representing the distribution of log entries by tag.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogEntryTagDistributionResource {
        ///Count of log entries for the specified tag.
        pub count: std::num::NonZeroU64,
        ///Tag associated with the distribution.
        pub tag: LogManagementLogStatisticsLogEntryTagDistributionResourceTag,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogStatisticsLogEntryTagDistributionResourceType,
    }
    impl From<&LogManagementLogStatisticsLogEntryTagDistributionResource>
        for LogManagementLogStatisticsLogEntryTagDistributionResource
    {
        fn from(value: &LogManagementLogStatisticsLogEntryTagDistributionResource) -> Self {
            value.clone()
        }
    }
    ///Collection of LogEntryTagDistributionResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResource {
        ///Array of LogEntryTagDistributionResource.
        pub items: Vec<
            LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItem,
        >,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceType,
    }
    impl From<&LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResource>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResource
    {
        fn from(
            value: &LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResource,
        ) -> Self {
            value.clone()
        }
    }
    ///Resource representing the distribution of log entries by tag.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItem {
        ///Count of log entries for the specified tag.
        pub count: std::num::NonZeroU64,
        ///Tag associated with the distribution.
        pub tag: LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemTag,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemType,
    }
    impl From<&LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItem>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItem
    {
        fn from(
            value: &LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItem,
        ) -> Self {
            value.clone()
        }
    }
    ///Tag associated with the distribution.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemTag(
        String,
    );
    impl std::ops::Deref
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemTag
    {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl
        From<
            LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemTag,
        > for String
    {
        fn from(
            value: LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemTag,
        ) -> Self {
            value.0
        }
    }
    impl From<
        &LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemTag,
    >
    for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemTag {
        fn from(
            value: &LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemTag,
        ) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemTag
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemTag
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemTag
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemTag
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemTag
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemType {
        #[serde(rename = "log_entry_tag_distribution")]
        LogEntryTagDistribution,
    }
    impl From<
        &LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemType,
    >
    for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemType {
        fn from(
            value: &LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemType
    {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntryTagDistribution => "log_entry_tag_distribution".to_string(),
            }
        }
    }
    impl std::str::FromStr
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemType
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_tag_distribution" => Ok(Self::LogEntryTagDistribution),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceType>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceType
    {
        fn from(
            value: &LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceType
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Tag associated with the distribution.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct LogManagementLogStatisticsLogEntryTagDistributionResourceTag(String);
    impl std::ops::Deref for LogManagementLogStatisticsLogEntryTagDistributionResourceTag {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<LogManagementLogStatisticsLogEntryTagDistributionResourceTag> for String {
        fn from(value: LogManagementLogStatisticsLogEntryTagDistributionResourceTag) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogStatisticsLogEntryTagDistributionResourceTag>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceTag
    {
        fn from(value: &LogManagementLogStatisticsLogEntryTagDistributionResourceTag) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for LogManagementLogStatisticsLogEntryTagDistributionResourceTag {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            if value.len() < 1usize {
                return Err("shorter than 1 characters");
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogStatisticsLogEntryTagDistributionResourceTag {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceTag
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceTag
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for LogManagementLogStatisticsLogEntryTagDistributionResourceTag {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogEntryTagDistributionResourceType {
        #[serde(rename = "log_entry_tag_distribution")]
        LogEntryTagDistribution,
    }
    impl From<&LogManagementLogStatisticsLogEntryTagDistributionResourceType>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceType
    {
        fn from(value: &LogManagementLogStatisticsLogEntryTagDistributionResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogStatisticsLogEntryTagDistributionResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogEntryTagDistribution => "log_entry_tag_distribution".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogStatisticsLogEntryTagDistributionResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_entry_tag_distribution" => Ok(Self::LogEntryTagDistribution),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogStatisticsLogEntryTagDistributionResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogEntryTagDistributionResourceType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Resource representing the frequency count of logs.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogFrequencyCountResource {
        ///Count of logs for the specified date.
        pub count: std::num::NonZeroU64,
        ///Date associated with the frequency count.
        pub date: chrono::DateTime<chrono::offset::Utc>,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogStatisticsLogFrequencyCountResourceType,
    }
    impl From<&LogManagementLogStatisticsLogFrequencyCountResource>
        for LogManagementLogStatisticsLogFrequencyCountResource
    {
        fn from(value: &LogManagementLogStatisticsLogFrequencyCountResource) -> Self {
            value.clone()
        }
    }
    ///Collection of LogFrequencyCountResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogFrequencyCountResourceCollectionResource {
        ///Array of LogFrequencyCountResource.
        pub items:
            Vec<LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItem>,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceType,
    }
    impl From<&LogManagementLogStatisticsLogFrequencyCountResourceCollectionResource>
        for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResource
    {
        fn from(
            value: &LogManagementLogStatisticsLogFrequencyCountResourceCollectionResource,
        ) -> Self {
            value.clone()
        }
    }
    ///Resource representing the frequency count of logs.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItem {
        ///Count of logs for the specified date.
        pub count: std::num::NonZeroU64,
        ///Date associated with the frequency count.
        pub date: chrono::DateTime<chrono::offset::Utc>,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_:
            LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItemType,
    }
    impl From<&LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItem>
        for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItem
    {
        fn from(
            value: &LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItem,
        ) -> Self {
            value.clone()
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItemType {
        #[serde(rename = "log_frequency_count")]
        LogFrequencyCount,
    }
    impl From<&LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItemType>
        for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItemType
    {
        fn from(
            value: &LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItemType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString
        for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItemType
    {
        fn to_string(&self) -> String {
            match *self {
                Self::LogFrequencyCount => "log_frequency_count".to_string(),
            }
        }
    }
    impl std::str::FromStr
        for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItemType
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_frequency_count" => Ok(Self::LogFrequencyCount),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceType>
        for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceType
    {
        fn from(
            value: &LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr
        for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceType
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogFrequencyCountResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogFrequencyCountResourceType {
        #[serde(rename = "log_frequency_count")]
        LogFrequencyCount,
    }
    impl From<&LogManagementLogStatisticsLogFrequencyCountResourceType>
        for LogManagementLogStatisticsLogFrequencyCountResourceType
    {
        fn from(value: &LogManagementLogStatisticsLogFrequencyCountResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogStatisticsLogFrequencyCountResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogFrequencyCount => "log_frequency_count".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogStatisticsLogFrequencyCountResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_frequency_count" => Ok(Self::LogFrequencyCount),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogStatisticsLogFrequencyCountResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogStatisticsLogFrequencyCountResourceType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogStatisticsLogFrequencyCountResourceType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Resource representing the statistics of logs by level.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogLevelStatisticsResource {
        ///Count of logs for the specified level.
        pub count: std::num::NonZeroU64,
        pub level: LogManagementLogStatisticsLogLevelStatisticsResourceLevel,
        pub percentage: f64,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_: LogManagementLogStatisticsLogLevelStatisticsResourceType,
    }
    impl From<&LogManagementLogStatisticsLogLevelStatisticsResource>
        for LogManagementLogStatisticsLogLevelStatisticsResource
    {
        fn from(value: &LogManagementLogStatisticsLogLevelStatisticsResource) -> Self {
            value.clone()
        }
    }
    ///Collection of LogLevelStatisticsResource.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResource {
        ///Array of LogLevelStatisticsResource.
        pub items:
            Vec<LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItem>,
        ///Type identifier for the collection.
        #[serde(rename = "type")]
        pub type_: LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceType,
    }
    impl From<&LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResource>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResource
    {
        fn from(
            value: &LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResource,
        ) -> Self {
            value.clone()
        }
    }
    ///Resource representing the statistics of logs by level.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItem {
        ///Count of logs for the specified level.
        pub count: std::num::NonZeroU64,
        pub level:
            LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevel,
        pub percentage: f64,
        ///Type identifier for the resource.
        #[serde(rename = "type")]
        pub type_:
            LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemType,
    }
    impl From<&LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItem>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItem
    {
        fn from(
            value: &LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItem,
        ) -> Self {
            value.clone()
        }
    }
    ///Level associated with the statistics.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevel {
        ///Name of the level.
        pub name: LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelName,
        ///Value of the level.
        pub value: LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelValue,
    }
    impl From<&LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevel>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevel
    {
        fn from(
            value: &LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevel,
        ) -> Self {
            value.clone()
        }
    }
    ///Name of the level.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelName {
        Debug,
        Info,
        Notice,
        Warning,
        Error,
        Critical,
        Alert,
        Emergency,
    }
    impl From<
        &LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelName,
    >
    for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelName {
        fn from(
            value: &LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelName,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelName
    {
        fn to_string(&self) -> String {
            match *self {
                Self::Debug => "Debug".to_string(),
                Self::Info => "Info".to_string(),
                Self::Notice => "Notice".to_string(),
                Self::Warning => "Warning".to_string(),
                Self::Error => "Error".to_string(),
                Self::Critical => "Critical".to_string(),
                Self::Alert => "Alert".to_string(),
                Self::Emergency => "Emergency".to_string(),
            }
        }
    }
    impl std::str::FromStr
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelName
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "Debug" => Ok(Self::Debug),
                "Info" => Ok(Self::Info),
                "Notice" => Ok(Self::Notice),
                "Warning" => Ok(Self::Warning),
                "Error" => Ok(Self::Error),
                "Critical" => Ok(Self::Critical),
                "Alert" => Ok(Self::Alert),
                "Emergency" => Ok(Self::Emergency),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelName
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelName
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelName
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Value of the level.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelValue(
        i64,
    );
    impl std::ops::Deref
    for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelValue {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<
        LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelValue,
    > for i64 {
        fn from(
            value: LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelValue,
        ) -> Self {
            value.0
        }
    }
    impl From<
        &LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelValue,
    >
    for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelValue {
        fn from(
            value: &LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelValue,
        ) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64>
    for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelValue {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64]
                .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de>
    for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemLevelValue {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| { <D::Error as serde::de::Error>::custom(e.to_string()) })
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemType {
        #[serde(rename = "log_level_statistics")]
        LogLevelStatistics,
    }
    impl From<&LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemType>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemType
    {
        fn from(
            value: &LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemType
    {
        fn to_string(&self) -> String {
            match *self {
                Self::LogLevelStatistics => "log_level_statistics".to_string(),
            }
        }
    }
    impl std::str::FromStr
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemType
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_level_statistics" => Ok(Self::LogLevelStatistics),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceItemsItemType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Type identifier for the collection.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceType {
        #[serde(rename = "collection")]
        Collection,
    }
    impl From<&LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceType>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceType
    {
        fn from(
            value: &LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceType,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::Collection => "collection".to_string(),
            }
        }
    }
    impl std::str::FromStr
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceType
    {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "collection" => Ok(Self::Collection),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogLevelStatisticsResourceCollectionResourceType
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Level associated with the statistics.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogManagementLogStatisticsLogLevelStatisticsResourceLevel {
        ///Name of the level.
        pub name: LogManagementLogStatisticsLogLevelStatisticsResourceLevelName,
        ///Value of the level.
        pub value: LogManagementLogStatisticsLogLevelStatisticsResourceLevelValue,
    }
    impl From<&LogManagementLogStatisticsLogLevelStatisticsResourceLevel>
        for LogManagementLogStatisticsLogLevelStatisticsResourceLevel
    {
        fn from(value: &LogManagementLogStatisticsLogLevelStatisticsResourceLevel) -> Self {
            value.clone()
        }
    }
    ///Name of the level.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogLevelStatisticsResourceLevelName {
        Debug,
        Info,
        Notice,
        Warning,
        Error,
        Critical,
        Alert,
        Emergency,
    }
    impl From<&LogManagementLogStatisticsLogLevelStatisticsResourceLevelName>
        for LogManagementLogStatisticsLogLevelStatisticsResourceLevelName
    {
        fn from(value: &LogManagementLogStatisticsLogLevelStatisticsResourceLevelName) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogStatisticsLogLevelStatisticsResourceLevelName {
        fn to_string(&self) -> String {
            match *self {
                Self::Debug => "Debug".to_string(),
                Self::Info => "Info".to_string(),
                Self::Notice => "Notice".to_string(),
                Self::Warning => "Warning".to_string(),
                Self::Error => "Error".to_string(),
                Self::Critical => "Critical".to_string(),
                Self::Alert => "Alert".to_string(),
                Self::Emergency => "Emergency".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogStatisticsLogLevelStatisticsResourceLevelName {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "Debug" => Ok(Self::Debug),
                "Info" => Ok(Self::Info),
                "Notice" => Ok(Self::Notice),
                "Warning" => Ok(Self::Warning),
                "Error" => Ok(Self::Error),
                "Critical" => Ok(Self::Critical),
                "Alert" => Ok(Self::Alert),
                "Emergency" => Ok(Self::Emergency),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogStatisticsLogLevelStatisticsResourceLevelName {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
        for LogManagementLogStatisticsLogLevelStatisticsResourceLevelName
    {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
        for LogManagementLogStatisticsLogLevelStatisticsResourceLevelName
    {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Value of the level.
    #[derive(Clone, Debug, Serialize)]
    pub struct LogManagementLogStatisticsLogLevelStatisticsResourceLevelValue(i64);
    impl std::ops::Deref for LogManagementLogStatisticsLogLevelStatisticsResourceLevelValue {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<LogManagementLogStatisticsLogLevelStatisticsResourceLevelValue> for i64 {
        fn from(value: LogManagementLogStatisticsLogLevelStatisticsResourceLevelValue) -> Self {
            value.0
        }
    }
    impl From<&LogManagementLogStatisticsLogLevelStatisticsResourceLevelValue>
        for LogManagementLogStatisticsLogLevelStatisticsResourceLevelValue
    {
        fn from(value: &LogManagementLogStatisticsLogLevelStatisticsResourceLevelValue) -> Self {
            value.clone()
        }
    }
    impl std::convert::TryFrom<i64> for LogManagementLogStatisticsLogLevelStatisticsResourceLevelValue {
        type Error = &'static str;
        fn try_from(value: i64) -> Result<Self, &'static str> {
            if ![
                100_i64, 200_i64, 250_i64, 300_i64, 400_i64, 500_i64, 550_i64, 600_i64,
            ]
            .contains(&value)
            {
                Err("invalid value")
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> serde::Deserialize<'de>
        for LogManagementLogStatisticsLogLevelStatisticsResourceLevelValue
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
        }
    }
    ///Type identifier for the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LogManagementLogStatisticsLogLevelStatisticsResourceType {
        #[serde(rename = "log_level_statistics")]
        LogLevelStatistics,
    }
    impl From<&LogManagementLogStatisticsLogLevelStatisticsResourceType>
        for LogManagementLogStatisticsLogLevelStatisticsResourceType
    {
        fn from(value: &LogManagementLogStatisticsLogLevelStatisticsResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for LogManagementLogStatisticsLogLevelStatisticsResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::LogLevelStatistics => "log_level_statistics".to_string(),
            }
        }
    }
    impl std::str::FromStr for LogManagementLogStatisticsLogLevelStatisticsResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "log_level_statistics" => Ok(Self::LogLevelStatistics),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for LogManagementLogStatisticsLogLevelStatisticsResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for LogManagementLogStatisticsLogLevelStatisticsResourceType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for LogManagementLogStatisticsLogLevelStatisticsResourceType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///Resource representing the ping
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PingResponse {
        ///A random quote from Hannibal, The Carthaginian General.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub quote: Option<String>,
        ///The date and time
        pub time: chrono::DateTime<chrono::offset::Utc>,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: PingResponseType,
    }
    impl From<&PingResponse> for PingResponse {
        fn from(value: &PingResponse) -> Self {
            value.clone()
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum PingResponseType {
        #[serde(rename = "ping")]
        Ping,
    }
    impl From<&PingResponseType> for PingResponseType {
        fn from(value: &PingResponseType) -> Self {
            value.clone()
        }
    }
    impl ToString for PingResponseType {
        fn to_string(&self) -> String {
            match *self {
                Self::Ping => "ping".to_string(),
            }
        }
    }
    impl std::str::FromStr for PingResponseType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "ping" => Ok(Self::Ping),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for PingResponseType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for PingResponseType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for PingResponseType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///The date and time
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SharedDateTime(pub chrono::DateTime<chrono::offset::Utc>);
    impl std::ops::Deref for SharedDateTime {
        type Target = chrono::DateTime<chrono::offset::Utc>;
        fn deref(&self) -> &chrono::DateTime<chrono::offset::Utc> {
            &self.0
        }
    }
    impl From<SharedDateTime> for chrono::DateTime<chrono::offset::Utc> {
        fn from(value: SharedDateTime) -> Self {
            value.0
        }
    }
    impl From<&SharedDateTime> for SharedDateTime {
        fn from(value: &SharedDateTime) -> Self {
            value.clone()
        }
    }
    impl From<chrono::DateTime<chrono::offset::Utc>> for SharedDateTime {
        fn from(value: chrono::DateTime<chrono::offset::Utc>) -> Self {
            Self(value)
        }
    }
    impl std::str::FromStr for SharedDateTime {
        type Err = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }
    impl std::convert::TryFrom<&str> for SharedDateTime {
        type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for SharedDateTime {
        type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for SharedDateTime {
        type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl ToString for SharedDateTime {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }
    ///The universally unique identifier (UUID) representing an identity in the system.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SharedIdentity(pub uuid::Uuid);
    impl std::ops::Deref for SharedIdentity {
        type Target = uuid::Uuid;
        fn deref(&self) -> &uuid::Uuid {
            &self.0
        }
    }
    impl From<SharedIdentity> for uuid::Uuid {
        fn from(value: SharedIdentity) -> Self {
            value.0
        }
    }
    impl From<&SharedIdentity> for SharedIdentity {
        fn from(value: &SharedIdentity) -> Self {
            value.clone()
        }
    }
    impl From<uuid::Uuid> for SharedIdentity {
        fn from(value: uuid::Uuid) -> Self {
            Self(value)
        }
    }
    impl std::str::FromStr for SharedIdentity {
        type Err = <uuid::Uuid as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }
    impl std::convert::TryFrom<&str> for SharedIdentity {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for SharedIdentity {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for SharedIdentity {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl ToString for SharedIdentity {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SharedPaginationItemsPerPage(pub i64);
    impl std::ops::Deref for SharedPaginationItemsPerPage {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<SharedPaginationItemsPerPage> for i64 {
        fn from(value: SharedPaginationItemsPerPage) -> Self {
            value.0
        }
    }
    impl From<&SharedPaginationItemsPerPage> for SharedPaginationItemsPerPage {
        fn from(value: &SharedPaginationItemsPerPage) -> Self {
            value.clone()
        }
    }
    impl From<i64> for SharedPaginationItemsPerPage {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }
    impl std::str::FromStr for SharedPaginationItemsPerPage {
        type Err = <i64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }
    impl std::convert::TryFrom<&str> for SharedPaginationItemsPerPage {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for SharedPaginationItemsPerPage {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for SharedPaginationItemsPerPage {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl ToString for SharedPaginationItemsPerPage {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SharedPaginationPage(pub std::num::NonZeroU64);
    impl std::ops::Deref for SharedPaginationPage {
        type Target = std::num::NonZeroU64;
        fn deref(&self) -> &std::num::NonZeroU64 {
            &self.0
        }
    }
    impl From<SharedPaginationPage> for std::num::NonZeroU64 {
        fn from(value: SharedPaginationPage) -> Self {
            value.0
        }
    }
    impl From<&SharedPaginationPage> for SharedPaginationPage {
        fn from(value: &SharedPaginationPage) -> Self {
            value.clone()
        }
    }
    impl From<std::num::NonZeroU64> for SharedPaginationPage {
        fn from(value: std::num::NonZeroU64) -> Self {
            Self(value)
        }
    }
    impl std::str::FromStr for SharedPaginationPage {
        type Err = <std::num::NonZeroU64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }
    impl std::convert::TryFrom<&str> for SharedPaginationPage {
        type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for SharedPaginationPage {
        type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for SharedPaginationPage {
        type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl ToString for SharedPaginationPage {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }
    ///Resource representing the ping
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SharedPingResource {
        ///A random quote from Hannibal, The Carthaginian General.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub quote: Option<String>,
        ///The date and time
        pub time: chrono::DateTime<chrono::offset::Utc>,
        ///Type of the resource.
        #[serde(rename = "type")]
        pub type_: SharedPingResourceType,
    }
    impl From<&SharedPingResource> for SharedPingResource {
        fn from(value: &SharedPingResource) -> Self {
            value.clone()
        }
    }
    ///Type of the resource.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum SharedPingResourceType {
        #[serde(rename = "ping")]
        Ping,
    }
    impl From<&SharedPingResourceType> for SharedPingResourceType {
        fn from(value: &SharedPingResourceType) -> Self {
            value.clone()
        }
    }
    impl ToString for SharedPingResourceType {
        fn to_string(&self) -> String {
            match *self {
                Self::Ping => "ping".to_string(),
            }
        }
    }
    impl std::str::FromStr for SharedPingResourceType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "ping" => Ok(Self::Ping),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for SharedPingResourceType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for SharedPingResourceType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for SharedPingResourceType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Carthage API

The API Specification for Carthage

Version: 0.0.0*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }
    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "0.0.0"
    }
}
impl Client {
    /**Retrieve the ping status

    This endpoint is used to check the availability of the service. It returns a random quote from Hannibal, The Carthaginian General, along with the current time.

    Sends a `GET` request to `/ping`

    */
    pub async fn ping<'a>(&'a self) -> Result<ResponseValue<types::PingResponse>, Error<()>> {
        let url = format!("{}/ping", self.baseurl,);
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Collect Multiple Logs

    Collect multiple log entries, for multiple logs.

    Sends a `POST` request to `/log-management/collect`

    */
    pub async fn log_management_collect<'a>(
        &'a self,
        body: &'a types::LogManagementCollectBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/log-management/collect", self.baseurl,);
        let request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Log Collection

    Retrieve a collection of logs.

    Sends a `GET` request to `/log-management/log`

    Arguments:
    - `contains`
    - `from`
    - `items_per_page`: The number of items per page for pagination. Defaults to 20, with a maximum of 2000.
    - `levels`
    - `order`
    - `page`: The page number for pagination. Defaults to 1.
    - `sort_by`
    - `to`
    */
    pub async fn log_management_get_log_collection<'a>(
        &'a self,
        contains: Option<&'a str>,
        from: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
        items_per_page: Option<i64>,
        levels: Option<&'a Vec<types::LogManagementGetLogCollectionLevelsItem>>,
        order: Option<types::LogManagementGetLogCollectionOrder>,
        page: Option<std::num::NonZeroU64>,
        sort_by: Option<types::LogManagementGetLogCollectionSortBy>,
        to: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
    ) -> Result<ResponseValue<types::LogManagementGetLogCollectionResponse>, Error<()>> {
        let url = format!("{}/log-management/log", self.baseurl,);
        let mut query = Vec::with_capacity(8usize);
        if let Some(v) = &contains {
            query.push(("contains", v.to_string()));
        }
        if let Some(v) = &from {
            query.push(("from", v.to_string()));
        }
        if let Some(v) = &items_per_page {
            query.push(("items_per_page", v.to_string()));
        }
        if let Some(v) = &levels {
            {
                let mut query_values = Vec::new();
                for carthage_array_value in *v {
                    query_values.push(("levels[]", carthage_array_value.to_string()));
                }
                query.extend(query_values);
            };
        }
        if let Some(v) = &order {
            query.push(("order", v.to_string()));
        }
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &sort_by {
            query.push(("sort_by", v.to_string()));
        }
        if let Some(v) = &to {
            query.push(("to", v.to_string()));
        }
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create Log

    Create a new log.

    Sends a `POST` request to `/log-management/log`

    */
    pub async fn log_management_create_log<'a>(
        &'a self,
        body: &'a types::LogManagementCreateLogBody,
    ) -> Result<ResponseValue<types::LogManagementCreateLogResponse>, Error<()>> {
        let url = format!("{}/log-management/log", self.baseurl,);
        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            409u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Log

    Get a log by its identity.

    Sends a `GET` request to `/log-management/log/{identity}`

    */
    pub async fn log_management_get_log<'a>(
        &'a self,
        identity: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::LogManagementGetLogResponse>, Error<()>> {
        let url = format!(
            "{}/log-management/log/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
        );
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete Log

    Delete a log by identity.

    Sends a `DELETE` request to `/log-management/log/{identity}`

    */
    pub async fn log_management_delete_log<'a>(
        &'a self,
        identity: &'a uuid::Uuid,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/log-management/log/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Log Namespace Collection

    Retrieve a collection of log namespaces.

    Sends a `GET` request to `/log-management/log/namespace`

    */
    pub async fn log_management_get_log_namespace_collection<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::LogManagementGetLogNamespaceCollectionResponse>, Error<()>>
    {
        let url = format!("{}/log-management/log/namespace", self.baseurl,);
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Log Entry Collection

    Retrieve a collection of log entries.

    Sends a `GET` request to `/log-management/log/entry`

    Arguments:
    - `after`
    - `before`
    - `items_per_page`: The number of items per page for pagination. Defaults to 20, with a maximum of 2000.
    - `log_identity`
    - `order`
    - `page`: The page number for pagination. Defaults to 1.
    - `source`
    */
    pub async fn log_management_get_log_entry_collection<'a>(
        &'a self,
        after: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
        before: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
        items_per_page: Option<i64>,
        log_identity: Option<&'a uuid::Uuid>,
        order: Option<types::LogManagementGetLogEntryCollectionOrder>,
        page: Option<std::num::NonZeroU64>,
        source: Option<&'a str>,
    ) -> Result<ResponseValue<types::LogManagementGetLogEntryCollectionResponse>, Error<()>> {
        let url = format!("{}/log-management/log/entry", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &after {
            query.push(("after", v.to_string()));
        }
        if let Some(v) = &before {
            query.push(("before", v.to_string()));
        }
        if let Some(v) = &items_per_page {
            query.push(("items_per_page", v.to_string()));
        }
        if let Some(v) = &log_identity {
            query.push(("log_identity", v.to_string()));
        }
        if let Some(v) = &order {
            query.push(("order", v.to_string()));
        }
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &source {
            query.push(("source", v.to_string()));
        }
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create Log Entry

    Create a new log entry.

    Sends a `POST` request to `/log-management/log/entry`

    */
    pub async fn log_management_create_log_entry<'a>(
        &'a self,
        body: &'a types::LogManagementCreateLogEntryBody,
    ) -> Result<ResponseValue<types::LogManagementCreateLogEntryResponse>, Error<()>> {
        let url = format!("{}/log-management/log/entry", self.baseurl,);
        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Log Entry

    Get a log entry by its identity.

    Sends a `GET` request to `/log-management/log/entry/{identity}`

    */
    pub async fn log_management_get_log_entry<'a>(
        &'a self,
        identity: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::LogManagementGetLogEntryResponse>, Error<()>> {
        let url = format!(
            "{}/log-management/log/entry/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
        );
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete Log Entry

    Delete a log entry by identity.

    Sends a `DELETE` request to `/log-management/log/entry/{identity}`

    */
    pub async fn log_management_delete_log_entry<'a>(
        &'a self,
        identity: &'a uuid::Uuid,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/log-management/log/entry/{}",
            self.baseurl,
            encode_path(&identity.to_string()),
        );
        let request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Log Entry Tag Collection

    Retrieve a collection of log entry tags.

    Sends a `GET` request to `/log-management/log/entry/tag`

    */
    pub async fn log_management_get_log_entry_tag_collection<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::LogManagementGetLogEntryTagCollectionResponse>, Error<()>>
    {
        let url = format!("{}/log-management/log/entry/tag", self.baseurl,);
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Log Entry Source Collection

    Retrieve a collection of log entry sources.

    Sends a `GET` request to `/log-management/log/entry/source`

    */
    pub async fn log_management_get_log_entry_source_collection<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::LogManagementGetLogEntrySourceCollectionResponse>, Error<()>>
    {
        let url = format!("{}/log-management/log/entry/source", self.baseurl,);
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Collect Log Entries

    Collect log entries from a log.

    Sends a `POST` request to `/log-management/log/collect`

    */
    pub async fn log_management_collect_log<'a>(
        &'a self,
        body: &'a types::LogManagementCollectLogBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/log-management/log/collect", self.baseurl,);
        let request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Log Entry Frequency Count

    Get the frequency count of log entries.

    Sends a `GET` request to `/log-management/log/statistic/entry-frequency-count/{frequency}`

    */
    pub async fn log_management_get_log_entry_frequency_count_collection<'a>(
        &'a self,
        frequency: types::LogManagementGetLogEntryFrequencyCountCollectionFrequency,
    ) -> Result<
        ResponseValue<types::LogManagementGetLogEntryFrequencyCountCollectionResponse>,
        Error<()>,
    > {
        let url = format!(
            "{}/log-management/log/statistic/entry-frequency-count/{}",
            self.baseurl,
            encode_path(&frequency.to_string()),
        );
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Log Entry Source Frequency

    Get the frequency count of log entry sources.

    Sends a `GET` request to `/log-management/log/statistic/entry-source-frequency`

    */
    pub async fn log_management_get_log_entry_source_frequency_collection<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::LogManagementGetLogEntrySourceFrequencyCollectionResponse>,
        Error<()>,
    > {
        let url = format!(
            "{}/log-management/log/statistic/entry-source-frequency",
            self.baseurl,
        );
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Log Entry Tag Distribution

    Get the distribution of log entry tags.

    Sends a `GET` request to `/log-management/log/statistic/entry-tag-distribution`

    */
    pub async fn log_management_get_log_entry_tag_distribution_collection<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::LogManagementGetLogEntryTagDistributionCollectionResponse>,
        Error<()>,
    > {
        let url = format!(
            "{}/log-management/log/statistic/entry-tag-distribution",
            self.baseurl,
        );
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Log Frequency Count

    Get the frequency count of logs.

    Sends a `GET` request to `/log-management/log/statistic/frequency-count/{frequency}`

    */
    pub async fn log_management_get_log_frequency_count_collection<'a>(
        &'a self,
        frequency: types::LogManagementGetLogFrequencyCountCollectionFrequency,
    ) -> Result<ResponseValue<types::LogManagementGetLogFrequencyCountCollectionResponse>, Error<()>>
    {
        let url = format!(
            "{}/log-management/log/statistic/frequency-count/{}",
            self.baseurl,
            encode_path(&frequency.to_string()),
        );
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get Log Level Statistics

    Get the statistics of log levels.

    Sends a `GET` request to `/log-management/log/statistic/level-statistics`

    */
    pub async fn log_management_get_log_level_statistics_collection<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::LogManagementGetLogLevelStatisticsCollectionResponse>, Error<()>>
    {
        let url = format!(
            "{}/log-management/log/statistic/level-statistics",
            self.baseurl,
        );
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
pub mod prelude {
    pub use super::Client;
}
