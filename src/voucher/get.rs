use std::io::Read;
use hyper::Url;
use hyper::method::Method;
use serde_json;

use request::VoucherifyRequest;
use voucher::Voucher;
use utils::error::VoucherifyError;

pub struct VoucherGetRequest {
    request: VoucherifyRequest,

    voucher_id: String,
}

impl VoucherGetRequest {
    pub fn new(request: VoucherifyRequest, voucher_id: &str) -> VoucherGetRequest {
        VoucherGetRequest {
            request: request,

            voucher_id: voucher_id.to_string(),
        }
    }

    pub fn send(&mut self) -> Result<Voucher, VoucherifyError> {
        let url = try!(Url::parse(format!("{}/{}",
                                          "https://api.voucherify.io/v1/vouchers",
                                          self.voucher_id)
            .as_str()));

        let mut response = try!(self.request.execute(Method::Get, url));

        let mut json = String::new();
        let _ = try!(response.read_to_string(&mut json));

        match serde_json::from_str(json.as_str()) {
            Ok(voucher) => Ok(voucher),
            Err(err) => Err(VoucherifyError::JsonParse(err)),
        }
    }
}
