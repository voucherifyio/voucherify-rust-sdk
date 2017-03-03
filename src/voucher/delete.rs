use hyper::Url;
use hyper::status::StatusCode;

use request::VoucherifyRequest;

pub struct VoucherDeleteRequest {
    request: VoucherifyRequest,

    force: bool,
    voucher_id: String,
}

impl  VoucherDeleteRequest {
    pub fn new(request: VoucherifyRequest, voucher_id: &str) -> VoucherDeleteRequest {
        VoucherDeleteRequest {
            request: request,

            force: false,
            voucher_id: voucher_id.to_string(),
        }
    }

    pub fn force(&mut self, force: bool) -> &mut VoucherDeleteRequest {
        self.force = force;
        self
    }

    pub fn send(&mut self) -> Result<bool, String> {
        let mut url = match Url::parse(format!("{}/{}", "https://api.voucherify.io/v1/vouchers", self.voucher_id).as_str()) {
            Ok(u) => u,
            Err(_) => return Err("Invalid voucher Id".to_string()),
        };

        if self.force {
            url.query_pairs_mut().append_pair("force", "true");
        }

        let response = match self.request.delete(url)
            .execute() {
                Ok(r) => r,
                Err(err) => return Err(err.to_string()),
            };

        match response.status {
            StatusCode::Ok => Ok(true),
            _ => Err("Something went wrong".to_string())
        }
    }
}
