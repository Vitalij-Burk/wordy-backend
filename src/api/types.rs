use reqwest::StatusCode;

pub type HandlerError = (StatusCode, &'static str);
