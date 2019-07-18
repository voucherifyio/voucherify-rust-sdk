use std::io::Read;
use hyper::Url;
use hyper::method::Method;

use request::VoucherifyRequest;
use utils::error::VoucherifyError;

pub struct VoucherEnableRequest {
    request: VoucherifyRequest,
    app_url: String,

    voucher_id: String,
}

impl VoucherEnableRequest {
    pub fn new(request: VoucherifyRequest, voucher_id: &str, app_url: String) -> VoucherEnableRequest {
        VoucherEnableRequest {
            request: request,
            app_url: app_url,

            voucher_id: voucher_id.to_string(),
        }
    }

    pub fn send(&mut self) -> Result<bool, VoucherifyError> {
        let url = try!(Url::parse(format!("{}/v1/vouchers/{}/enable",
                                          self.app_url,
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
