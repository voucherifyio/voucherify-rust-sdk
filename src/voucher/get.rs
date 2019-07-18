use std::io::Read;
use hyper::Url;
use hyper::method::Method;
use serde_json;

use request::VoucherifyRequest;
use utils::error::VoucherifyError;
use voucher::Voucher;

pub struct VoucherGetRequest {
    request: VoucherifyRequest,
    app_url: String,

    voucher_id: String,
}

impl VoucherGetRequest {
    pub fn new(request: VoucherifyRequest, voucher_id: &str, app_url: String) -> VoucherGetRequest {
        VoucherGetRequest {
            request: request,
            app_url: app_url,

            voucher_id: voucher_id.to_string(),
        }
    }

    pub fn send(&mut self) -> Result<Voucher, VoucherifyError> {
        let url = try!(Url::parse(format!("{}/v1/vouchers/{}",
                                          self.app_url,
                                          self.voucher_id)
            .as_str()));

        let mut response = try!(self.request.execute(Method::Get, url));

        let mut json = String::new();
        let _ = try!(response.read_to_string(&mut json));

        if !response.status.is_success() {
            return Err(VoucherifyError::ResponseError(json))
        }

        match serde_json::from_str(json.as_str()) {
            Ok(voucher) => Ok(voucher),
            Err(err) => Err(VoucherifyError::JsonParse(err)),
        }
    }
}
