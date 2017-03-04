use std::io::Read;
use hyper::Url;
use hyper::method::Method;
use serde_json;

use request::VoucherifyRequest;
use utils::error::VoucherifyError;
use voucher::Voucher;

pub struct VoucherListRequest {
    request: VoucherifyRequest,

    limit: u32,
    page: u32,
    category: String,
    campaign: String,
}

impl VoucherListRequest {
    pub fn new(request: VoucherifyRequest) -> VoucherListRequest {
        VoucherListRequest {
            request: request,
            limit: 10,
            page: 1,
            category: String::new(),
            campaign: String::new(),
        }
    }

    pub fn limit(&mut self, limit: u32) -> &mut VoucherListRequest {
        self.limit = limit;
        self
    }

    pub fn page(&mut self, page: u32) -> &mut VoucherListRequest {
        self.page = page;
        self
    }

    pub fn category(&mut self, category: &str) -> &mut VoucherListRequest {
        self.category = category.to_string();
        self
    }

    pub fn campaign(&mut self, campaign: &str) -> &mut VoucherListRequest {
        self.campaign = campaign.to_string();
        self
    }

    pub fn send(&mut self) -> Result<Vec<Voucher>, VoucherifyError> {
        let mut url = try!(Url::parse("https://api.voucherify.io/v1/vouchers"));
        url.query_pairs_mut()
            .clear()
            .append_pair("limit", format!("{}", self.limit).as_str())
            .append_pair("page", format!("{}", self.page).as_str());

        if !self.category.is_empty() {
            url.query_pairs_mut().append_pair("category", self.category.as_str());
        }

        if !self.campaign.is_empty() {
            url.query_pairs_mut().append_pair("campaign", self.campaign.as_str());
        }

        let mut response = try!(self.request.execute(Method::Get, url));

        let mut json = String::new();
        let _ = try!(response.read_to_string(&mut json));

        match serde_json::from_str(json.as_str()) {
            Ok(voucher) => Ok(voucher),
            Err(err) => Err(VoucherifyError::JsonParse(err)),
        }
    }
}
