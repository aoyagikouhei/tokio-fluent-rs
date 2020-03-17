use tokio_fluent::{FluentClient, FluentError};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), FluentError> {
    let mut record = HashMap::new();
    record.insert("key".to_string(), "value".to_string());

    let client = FluentClient::new("0.0.0.0:24224");
    client.message("test.aaa", &record).await
}
