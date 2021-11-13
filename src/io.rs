use std::fmt::{Display, Debug, Formatter};
use serde_json::json;
use std::error::Error;
use clap::{Parser};
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////

/// This app is used to format a welcome message :)
#[derive(Parser, Deserialize, Clone, Debug)]
#[clap(version = "1.0")]
pub struct AppInput {
    #[serde(rename = "firstName")]
    pub first_name: String,
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// This class is used only by the AWS Lambda output. This will be serialized into
/// JSON format.
#[derive(Serialize, Clone)]
pub struct AppOutput {
    pub message: String,
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Debug)]
pub struct AppError {
    pub message: String,
}

impl Display for AppError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let err_json = json!(self).to_string();
        write!(formatter, "{}", err_json)
    }
}

impl Error for AppError {
}

