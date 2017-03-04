use std::io;
use hyper::error;
use serde_json;

#[derive(Debug)]
pub enum VoucherifyError {
    Io(io::Error),
    UrlParse(error::ParseError),
    RequestError(error::Error),
    ResponseError(String),
    JsonParse(serde_json::Error),
}

impl From<io::Error> for VoucherifyError {
    fn from(err: io::Error) -> VoucherifyError {
        VoucherifyError::Io(err)
    }
}

impl From<error::ParseError> for VoucherifyError {
    fn from(err: error::ParseError) -> VoucherifyError {
        VoucherifyError::UrlParse(err)
    }
}

impl From<error::Error> for VoucherifyError {
    fn from(err: error::Error) -> VoucherifyError {
        VoucherifyError::RequestError(err)
    }
}

impl From<serde_json::Error> for VoucherifyError {
    fn from(err: serde_json::Error) -> VoucherifyError {
        VoucherifyError::JsonParse(err)
    }
}
