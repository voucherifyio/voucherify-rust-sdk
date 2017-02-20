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

    pub fn send(&self) -> Voucher {
        let url = Url::parse(("https://api.voucherify.io/v1/vouchers/".to_string() + self.voucher_id.as_str()).as_str()).unwrap();

        let mut response = self.request.execute(url);

        let mut json = String::new();
        response.read_to_string(&mut json);

        println!("{:?}", json);

        serde_json::from_str(json.as_str()).unwrap()
    }
}
