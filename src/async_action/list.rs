use std::io::Read;
use hyper::Url;
use hyper::method::Method;

use request::VoucherifyRequest;
use utils::error::VoucherifyError;
use async_action::AsyncActionList;

pub struct AsyncActionListRequest {
    request: VoucherifyRequest,
    app_url: String,

    limit: u32,
    end_date: String,
}

impl AsyncActionListRequest {
    pub fn new(request: VoucherifyRequest, app_url: String) -> AsyncActionListRequest {
        AsyncActionListRequest {
            request,
            app_url,

            limit: 100,
            end_date: String::new(),
        }
    }

    pub fn limit(&mut self, limit: u32) -> &mut AsyncActionListRequest {
        self.limit = limit;
        self
    }

    pub fn end_date(&mut self, end_date: &str) -> &mut AsyncActionListRequest {
        self.end_date = end_date.to_string();
        self
    }

    pub fn send(&mut self) -> Result<AsyncActionList, VoucherifyError> {
        let mut url = try!(Url::parse(format!("{}/v1/async-actions", self.app_url).as_str()));
        url.query_pairs_mut()
            .clear()
            .append_pair("limit", format!("{}", self.limit).as_str());

        if !self.end_date.is_empty() {
            url.query_pairs_mut().append_pair("end_date", self.end_date.as_str());
        }

        let mut response = try!(self.request.execute(Method::Get, url));

        let mut json = String::new();
        let _ = try!(response.read_to_string(&mut json));

        match serde_json::from_str(json.as_str()) {
            Ok(async_action_list) => Ok(async_action_list),
            Err(err) => Err(VoucherifyError::JsonParse(err)),
        }
    }
}
