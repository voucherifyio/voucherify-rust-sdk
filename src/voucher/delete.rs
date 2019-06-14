use hyper::Url;
use hyper::method::Method;

use request::VoucherifyRequest;
use utils::error::VoucherifyError;

pub struct VoucherDeleteRequest {
    request: VoucherifyRequest,
    app_url: String,

    force: bool,
    voucher_id: String,
}

impl VoucherDeleteRequest {
    pub fn new(request: VoucherifyRequest, voucher_id: &str, app_url: String) -> VoucherDeleteRequest {
        VoucherDeleteRequest {
            request: request,
            app_url: app_url,

            force: false,
            voucher_id: voucher_id.to_string(),
        }
    }

    pub fn force(&mut self, force: bool) -> &mut VoucherDeleteRequest {
        self.force = force;
        self
    }

    pub fn send(&mut self) -> Result<bool, VoucherifyError> {
        let mut url = try!(Url::parse(format!("{}/v1/vouchers/{}",
                                              self.app_url,
                                              self.voucher_id)
            .as_str()));

        if self.force {
            url.query_pairs_mut().append_pair("force", "true");
        }

        let response = try!(self.request.execute(Method::Delete, url));

        if !response.status.is_success() {
            return Err(VoucherifyError::ResponseError("Resource not found".to_string()));
        }

        Ok(true)
    }
}
