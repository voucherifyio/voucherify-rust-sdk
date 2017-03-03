use std::io::Read;
use std::collections::BTreeMap;
use hyper::Url;
use hyper::method::Method;
use serde_json;
use serde_json::Value;

use request::VoucherifyRequest;
use voucher::{Voucher, Gift};

pub struct VoucherUpdateRequest {
    request: VoucherifyRequest,

    voucher: Voucher,
}

impl VoucherUpdateRequest {
    pub fn new(request: VoucherifyRequest, voucher_id: &str) -> VoucherUpdateRequest {
        VoucherUpdateRequest {
            request: request,

            voucher: Voucher::new().code(voucher_id.to_string()).build(),
        }
    }

    pub fn category(&mut self, category: &str) -> &mut VoucherUpdateRequest {
        self.voucher.category = Some(category.to_string());
        self
    }

    pub fn start_date(&mut self, start_date: &str) -> &mut VoucherUpdateRequest {
        self.voucher.start_date = Some(start_date.to_string());
        self
    }

    pub fn expiration_date(&mut self, expiration_date: &str) -> &mut VoucherUpdateRequest {
        self.voucher.expiration_date = Some(expiration_date.to_string());
        self
    }

    pub fn active(&mut self, active: bool) -> &mut VoucherUpdateRequest {
        self.voucher.active = Some(active);
        self
    }

    pub fn additional_info(&mut self, additional_info: String) -> &mut VoucherUpdateRequest {
        self.voucher.additional_info = Some(additional_info);
        self
    }

    pub fn metadata(&mut self, metadata: BTreeMap<String, Value>) -> &mut VoucherUpdateRequest {
        self.voucher.metadata = Some(metadata);
        self
    }

    pub fn gift_amount(&mut self, amount: usize) -> &mut VoucherUpdateRequest {
        self.voucher.gift = Some(Gift::new(amount, 0));
        self
    }

    pub fn send(&mut self) -> Result<Voucher, String> {
        let voucher_id = self.voucher.code.clone();
        let url = match Url::parse(format!("{}/{}",
                                           "https://api.voucherify.io/v1/vouchers",
                                           voucher_id.unwrap())
            .as_str()) {
            Ok(u) => u,
            Err(_) => return Err("Invalid voucher Id".to_string()),
        };

        let payload = match serde_json::to_string(&self.voucher) {
            Ok(p) => p,
            Err(_) => return Err("Failed to parse object to JSON".to_string()),
        };

        let mut response = match self.request
            .payload(payload)
            .execute(Method::Put, url) {
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
            Err(err) => {
                println!("{:?}", err);
                Err("Failed to parse JSON".to_string())
            }
        }
    }
}
