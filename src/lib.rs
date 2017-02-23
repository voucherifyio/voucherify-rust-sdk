#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate hyper;
extern crate hyper_native_tls;

pub mod request;
pub mod voucher;
pub mod utils;

use request::VoucherifyRequest;
use voucher::create::VoucherCreateRequest;
use voucher::get::VoucherGetRequest;
use voucher::update::VoucherUpdateRequest;
use voucher::list::VoucherListRequest;
use voucher::Voucher;

pub struct Voucherify {
    api_key: String,
    api_user: String,
}

impl Voucherify {
    pub fn new(key: &str, user: &str) -> Voucherify {
        Voucherify {
            api_key: key.to_string(),
            api_user: user.to_string(),
        }
    }

    pub fn voucher_create(&self, voucher: Voucher) -> VoucherCreateRequest {
        let new_request = VoucherifyRequest::new(&self.api_key, &self.api_user);
        VoucherCreateRequest::new(new_request, voucher)
    }

    pub fn voucher_get(&self, voucher_id: &str) -> VoucherGetRequest {
        let new_request = VoucherifyRequest::new(&self.api_key, &self.api_user);
        VoucherGetRequest::new(new_request, voucher_id)
    }

    pub fn voucher_update(&self, voucher_id: &str) -> VoucherUpdateRequest {
        let new_request = VoucherifyRequest::new(&self.api_key, &self.api_user);
        VoucherUpdateRequest::new(new_request, voucher_id)
    }

    pub fn voucher_list(&self) -> VoucherListRequest {
        let new_request = VoucherifyRequest::new(&self.api_key, &self.api_user);
        VoucherListRequest::new(new_request)
    }
}
