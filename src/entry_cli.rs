mod io;
mod app;

use std::error::Error;
use io::{AppInput};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;

    let input = AppInput::parse();
    let result = app::main(input).await;

    // Define your output
    match result {
        Ok(result) => {
            println!("{}", result.message);
            Ok(())
        },
        Err(err) => {
            eprintln!("Error: {}", err.message);
            std::process::exit(1)
        },
    }
}
