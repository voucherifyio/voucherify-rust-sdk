pub mod create;
pub mod get;
pub mod update;
pub mod delete;
pub mod list;
pub mod enable;
pub mod disable;

use std::collections::BTreeMap;
use serde_json::Value;

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct VouchersList {
    pub vouchers: Vec<Voucher>
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Voucher {
    pub object: Option<String>,
    pub created_at: Option<String>,
    pub code: Option<String>,
    pub campaign: Option<String>,
    pub category: Option<String>,
    #[serde(rename = "type")]
    pub voucher_type: Option<VoucherType>,
    pub is_referral_code: Option<bool>,
    pub start_date: Option<String>,
    pub expiration_date: Option<String>,
    pub publish: Option<Publish>,
    pub redemption: Option<Redemption>,
    pub active: Option<bool>,
    pub additional_info: Option<String>,
    pub metadata: Option<BTreeMap<String, Value>>,
    pub discount: Option<Discount>,
    pub gift: Option<Gift>,
    pub assets: Option<Assets>,
    pub referrer_id: Option<String>,
}

impl Voucher {
    pub fn new() -> Voucher {
        Voucher::default()
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
        self.discount = Some(Discount::new(discount_type, amount));
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
    entries: Option<Vec<PublishEntry>>,
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
    redemption_entries: Option<Vec<RedemptionEntry>>,
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
    pub discount_type: DiscountType,
    pub amount_off: Option<u32>,
    pub percent_off: Option<u32>
}

impl Discount {
    pub fn new(discount_type: DiscountType, amount: u32) -> Discount {
        if discount_type == DiscountType::PERCENT {
            return Discount {
                discount_type: discount_type,
                amount_off: None,
                percent_off: Some(amount)
            }
        }

        Discount {
            discount_type: discount_type,
            amount_off: Some(amount),
            percent_off: None
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
    url: String,
}
