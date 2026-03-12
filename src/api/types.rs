use reqwest::StatusCode;

pub type HandlerError = (StatusCode, &'static str);
pub type JsonError = (StatusCode, &'static str);
