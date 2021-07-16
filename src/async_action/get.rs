use std::io::Read;
use hyper::Url;
use hyper::method::Method;
use serde_json;

use request::VoucherifyRequest;
use utils::error::VoucherifyError;
use async_action::AsyncAction;

pub struct AsyncActionGetRequest {
    request: VoucherifyRequest,
    app_url: String,

    id: String,
}

impl AsyncActionGetRequest {
    pub fn new(request: VoucherifyRequest, id: &str, app_url: String) -> AsyncActionGetRequest {
        AsyncActionGetRequest {
            request,
            app_url,

            id: id.to_string(),
        }
    }

    pub fn send(&mut self) -> Result<AsyncAction, VoucherifyError> {
        let url = try!(Url::parse(format!("{}/v1/async-actions/{}",
                                          self.app_url,
                                          self.id)
            .as_str()));

        let mut response = try!(self.request.execute(Method::Get, url));

        let mut json = String::new();
        let _ = try!(response.read_to_string(&mut json));

        if !response.status.is_success() {
            return Err(VoucherifyError::ResponseError(json))
        }

        match serde_json::from_str(json.as_str()) {
            Ok(async_action) => Ok(async_action),
            Err(err) => Err(VoucherifyError::JsonParse(err)),
        }
    }
}
