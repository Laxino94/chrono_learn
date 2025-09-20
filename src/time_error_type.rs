#![allow(dead_code)]
pub mod time_error_type {

    use chrono::format::ParseError;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum DateTimeError {
        #[error("time parse error: {0}")]
        ParseError(#[from] ParseError),

        #[error("format error: {0}")]
        FormatError(String),

        #[error("invalid datetime error: {0}")]
        InvalidDateTime(String),

        #[error("timezone error: {0}")]
        TimezoneError(String),
    }

    // The Display implementation is provided by the thiserror::Error derive macro.
}
