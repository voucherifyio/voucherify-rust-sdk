use std::io::Read;
use hyper::Url;
use hyper::method::Method;
use serde_json;

use request::VoucherifyRequest;
use voucher::Voucher;

pub struct VoucherCreateRequest {
    request: VoucherifyRequest,

    voucher: Voucher,
}

impl VoucherCreateRequest {
    pub fn new(request: VoucherifyRequest, voucher: Voucher) -> VoucherCreateRequest {
        VoucherCreateRequest {
            request: request,

            voucher: voucher,
        }
    }

    pub fn send(&mut self) -> Result<Voucher, String> {
        let url = Url::parse("https://api.voucherify.io/v1/vouchers").unwrap();

        let payload = match serde_json::to_string(&self.voucher) {
            Ok(p) => p,
            Err(_) => return Err("Failed to parse object to JSON".to_string()),
        };

        let mut response = match self.request
            .payload(payload)
            .execute(Method::Post, url) {
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
            Err(_) => Err("Failed to parse JSON".to_string()),
        }
    }
}
