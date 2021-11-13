mod io;
mod app;

use lambda_runtime::handler_fn;
use io::{AppInput, AppOutput, AppError};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda_runtime::run(handler_fn(my_handler)).await?;
    Ok(())
}

async fn my_handler(
    input: AppInput,
    context: lambda_runtime::Context,
) -> Result<AppOutput, lambda_runtime::Error> {
    let result = app::main(input).await?;
    Ok(result)
}
