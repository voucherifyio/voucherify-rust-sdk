#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_json;

pub mod request;
pub mod voucher;

use request::VoucherifyRequest;
use voucher::get::VoucherGetRequest;
use voucher::list::VoucherListRequest;

pub struct Voucherify {
    request: VoucherifyRequest,
}

impl Voucherify {
    pub fn new(key: &str, user: &str) -> Voucherify {
        Voucherify {
            request: VoucherifyRequest::new(key, user),
        }
    }

    pub fn voucher_get(&self, voucher_id: &str) -> VoucherGetRequest {
        VoucherGetRequest::new(&self.request, voucher_id)
    }

    pub fn voucher_list(&self) -> VoucherListRequest {
        VoucherListRequest::new(&self.request)
    }
}
