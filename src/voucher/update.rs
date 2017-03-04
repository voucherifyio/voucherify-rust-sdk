use std::io::Read;
use std::collections::BTreeMap;
use hyper::Url;
use hyper::method::Method;
use serde_json;
use serde_json::Value;

use request::VoucherifyRequest;
use utils::error::VoucherifyError;
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

    pub fn send(&mut self) -> Result<Voucher, VoucherifyError> {
        let voucher_id = self.voucher.code.clone();
        let url = try!(Url::parse(format!("{}/{}",
                                          "https://api.voucherify.io/v1/vouchers",
                                          voucher_id.unwrap())
            .as_str()));

        let payload = try!(serde_json::to_string(&self.voucher));

        let mut response = try!(self.request.payload(payload).execute(Method::Put, url));

        let mut json = String::new();
        let _ = try!(response.read_to_string(&mut json));

        match serde_json::from_str(json.as_str()) {
            Ok(voucher) => Ok(voucher),
            Err(err) => Err(VoucherifyError::JsonParse(err)),
        }
    }
}
