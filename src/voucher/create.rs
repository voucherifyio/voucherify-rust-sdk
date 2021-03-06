use std::io::Read;
use hyper::Url;
use hyper::method::Method;
use serde_json;

use request::VoucherifyRequest;
use utils::error::VoucherifyError;
use voucher::Voucher;

pub struct VoucherCreateRequest {
    request: VoucherifyRequest,
    app_url: String,

    voucher: Voucher,
}

impl VoucherCreateRequest {
    pub fn new(request: VoucherifyRequest, voucher: Voucher, app_url: String) -> VoucherCreateRequest {
        VoucherCreateRequest {
            request: request,
            app_url: app_url,

            voucher: voucher,
        }
    }

    pub fn send(&mut self) -> Result<Voucher, VoucherifyError> {
        let url = try!(Url::parse(format!("{}/v1/vouchers", self.app_url).as_str()));

        let payload = try!(serde_json::to_string(&self.voucher));

        let mut response = try!(self.request.payload(payload).execute(Method::Post, url));

        let mut json = String::new();
        let _ = try!(response.read_to_string(&mut json));

        if !response.status.is_success() {
            return Err(VoucherifyError::ResponseError(json))
        }

        match serde_json::from_str(json.as_str()) {
            Ok(voucher) => Ok(voucher),
            Err(err) => Err(VoucherifyError::JsonParse(err)),
        }
    }
}
