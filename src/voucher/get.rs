use std::io::Read;
use hyper::Url;
use serde_json;

use request::VoucherifyRequest;
use voucher::Voucher;

pub struct VoucherGetRequest<'a> {
    request: &'a VoucherifyRequest,

    voucher_id: String,
}

impl<'a> VoucherGetRequest<'a> {
    pub fn new(request: &'a VoucherifyRequest, voucher_id: &str) -> VoucherGetRequest<'a> {
        VoucherGetRequest {
            request: request,

            voucher_id: voucher_id.to_string(),
        }
    }

    pub fn send(&self) -> Result<Voucher, String> {
        let url = match Url::parse(format!("{}/{}", "https://api.voucherify.io/v1/vouchers", self.voucher_id).as_str()) {
            Ok(u) => u,
            Err(_) => return Err("Invalid voucher Id".to_string()),
        };

        let mut response = match self.request.execute(url) {
            Ok(r) => r,
            Err(err) => return Err(err.to_string()),
        };

        let mut json = String::new();
        match response.read_to_string(&mut json) {
            Err(_) => return Err("Failed to read JSON from response".to_string()),
            Ok(_) => (),
        };

        match serde_json::from_str(json.as_str()) {
            Ok(voucher) => Ok(voucher),
            Err(_) => Err("Faile to parse JSON".to_string())
        }
    }
}
