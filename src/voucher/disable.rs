use std::io::Read;
use hyper::Url;
use hyper::method::Method;

use request::VoucherifyRequest;
use utils::error::VoucherifyError;

pub struct VoucherDisableRequest {
    request: VoucherifyRequest,

    voucher_id: String,
}

impl VoucherDisableRequest {
    pub fn new(request: VoucherifyRequest, voucher_id: &str) -> VoucherDisableRequest {
        VoucherDisableRequest {
            request: request,

            voucher_id: voucher_id.to_string(),
        }
    }

    pub fn send(&mut self) -> Result<bool, VoucherifyError> {
        let url = try!(Url::parse(format!("{}/{}/disable",
                                          "https://api.voucherify.io/v1/vouchers",
                                          self.voucher_id)
                                  .as_str()));

        let mut response = try!(self.request.execute(Method::Post, url));

        let mut json = String::new();
        let _ = try!(response.read_to_string(&mut json));

        if !response.status.is_success() {
            return Err(VoucherifyError::ResponseError(json))
        }

        Ok(true)
    }
}
