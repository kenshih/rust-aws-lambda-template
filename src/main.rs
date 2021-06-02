use tokio::io::{AsyncReadExt, AsyncWriteExt};
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: Value, _: Context) -> Result<Value, Error> {
    print!("hello ken\n");
    Ok(json!({ "message": format!("Hello, {}!", "Ken") }))
}