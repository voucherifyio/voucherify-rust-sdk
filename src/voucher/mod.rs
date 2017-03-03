pub mod create;
pub mod get;
pub mod update;
pub mod delete;
pub mod list;

use std::collections::BTreeMap;
use serde_json::Value;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Voucher {
    object: Option<String>,
    created_at: Option<String>,
    code: Option<String>,
    campaign: Option<String>,
    category: Option<String>,
     #[serde(rename = "type")]
    voucher_type: Option<VoucherType>,
    is_referral_code: Option<bool>,
    start_date: Option<String>,
    expiration_date: Option<String>,
    publish: Option<Publish>,
    redemption: Option<Redemption>,
    active: Option<bool>,
    additional_info: Option<String>,
    metadata: Option<BTreeMap<String, Value>>,
    discount: Option<Discount>,
    gift: Option<Gift>,
    assets: Option<Assets>,
    referrer_id: Option<String>,
}

impl Voucher {
    pub fn new() -> Voucher {
        Voucher {
            object: None,
            created_at: None,
            code: None,
            campaign: None,
            category: None,
            voucher_type: None,
            is_referral_code: None,
            start_date: None,
            expiration_date: None,
            publish: None,
            redemption: None,
            active: None,
            additional_info: None,
            metadata: None,
            discount: None,
            gift: None,
            assets: None,
            referrer_id: None,
        }
    }

    pub fn code(mut self, code: String) -> Voucher {
        self.code = Some(code);
        self
    }

    pub fn category(mut self, category: String) -> Voucher {
        self.category = Some(category);
        self
    }

    pub fn voucher_type(mut self, voucher_type: VoucherType) -> Voucher {
        self.voucher_type = Some(voucher_type);
        self
    }

    pub fn start_date(mut self, start_date: String) -> Voucher {
        self.start_date = Some(start_date);
        self
    }

    pub fn expiration_date(mut self, expiration_date: String) -> Voucher {
        self.expiration_date = Some(expiration_date);
        self
    }

    pub fn discount(mut self, discount_type: DiscountType, amount: u32) -> Voucher {
        self.discount = Some(Discount::new(discount_type,  amount));
        self
    }

    pub fn build(self) -> Voucher {
        self
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum VoucherType {
    DISCOUNT_VOUCHER,
    GIFT_VOUCHER,
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Publish {
    count: u32,
    entries: Vec<PublishEntry>,
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct PublishEntry {
    customer_id: String,
    customer: String,
    channel: String,
    published_at: String,
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Redemption {
    redeemed_quantity: u32,
    redemption_entries: Vec<RedemptionEntry>,
    redeemed_amount: Option<u32>,
    quantity: Option<u32>,
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct RedemptionEntry {
    id: String,
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Discount {
     #[serde(rename = "type")]
    discount_type: DiscountType,
    amount_off: u32,
}

impl Discount {
    pub fn new(discount_type: DiscountType, amount: u32) -> Discount {
        Discount {
            discount_type: discount_type,
            amount_off: amount,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum DiscountType {
    AMOUNT,
    PERCENT,
    UNIT,
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Gift {
    amount: usize,
    balance: usize,
}

impl Gift {
    pub fn new(amount: usize, balance: usize) -> Gift {
        Gift {
            amount: amount,
            balance: balance,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Assets {
    qr: QrCode,
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QrCode {
    id: String,
    url: String
}
