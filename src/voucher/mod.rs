pub mod list;

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Voucher {
    object: String,
    created_at: String,
    code: String,
    campaign: String,
     #[serde(rename = "type")]
    voucher_type: String,
    is_referral_code: bool,
    publish: Publish,
    redemption: Redemption,
    active: bool,
    metadata: Option<Metadata>,
    discount: Option<Discount>,
    gift: Option<Gift>,
    assets: Assets,
    referrer_id: Option<String>,
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
    discount_type: String,
    amount_off: u32,
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
