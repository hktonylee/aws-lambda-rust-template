use crate::io::{AppInput, AppOutput, AppError};
use log::*;

pub async fn main(input: AppInput) -> Result<AppOutput, Box<AppError>> {
  if input.first_name == "" {
    error!("Empty first name in request");
    Err(Box::new(AppError {message: "Empty first name".to_string()}))

  } else if input.first_name.len() < 5 { 
    error!("First name is too short!");
    Err(Box::new(AppError {message: "First name is too short".to_string()}))

  } else {
    Ok(AppOutput {
      message: format!("Hi {}!", &input.first_name),
    })
  }
}
