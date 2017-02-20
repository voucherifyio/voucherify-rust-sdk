use std::io::Read;
use hyper::Url;
use serde_json;

use request::VoucherifyRequest;
use voucher::Voucher;

pub struct VoucherListRequest<'a> {
    request: &'a VoucherifyRequest,

    limit: u32,
    page: u32,
    category: String,
    campaign: String,
}

impl<'a> VoucherListRequest<'a> {
    pub fn new(request: &'a VoucherifyRequest) -> VoucherListRequest<'a> {
        VoucherListRequest {
            request: request,
            limit: 10,
            page: 1,
            category: String::new(),
            campaign: String::new(),
        }
    }

    pub fn limit(&mut self, limit: u32) -> &mut VoucherListRequest<'a> {
        self.limit = limit;
        self
    }

    pub fn page(&mut self, page: u32) -> &mut VoucherListRequest<'a> {
        self.page = page;
        self
    }

    pub fn category(&mut self, category: &str) -> &mut VoucherListRequest<'a> {
        self.category = category.to_string();
        self
    }

    pub fn campaign(&mut self, campaign: &str) -> &mut VoucherListRequest<'a> {
        self.campaign = campaign.to_string();
        self
    }

    pub fn send(&self) -> Vec<Voucher> {
        let mut url = Url::parse("https://api.voucherify.io/v1/vouchers").unwrap();
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

        let mut response = self.request.execute(url);

        let mut json = String::new();
        response.read_to_string(&mut json);

        serde_json::from_str(json.as_str()).unwrap()
    }
}
