pub mod create;
pub mod get;
pub mod list;

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Voucher {
    object: Option<String>,
    created_at: Option<String>,
    code: Option<String>,
    campaign: Option<String>,
     #[serde(rename = "type")]
    voucher_type: Option<VoucherType>,
    is_referral_code: Option<bool>,
    publish: Option<Publish>,
    redemption: Option<Redemption>,
    active: Option<bool>,
    metadata: Option<Metadata>,
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
            voucher_type: None,
            is_referral_code: None,
            publish: None,
            redemption: None,
            active: None,
            metadata: None,
            discount: None,
            gift: None,
            assets: None,
            referrer_id: None,
        }
    }

    pub fn voucher_type(mut self, voucher_type: VoucherType) -> Voucher {
        self.voucher_type = Some(voucher_type);
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
pub struct Metadata {
    test: bool,
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
    amount: u32,
    balance: u32,
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
