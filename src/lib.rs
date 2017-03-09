#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate hyper;
extern crate hyper_native_tls;

pub mod request;
pub mod voucher;
pub mod utils;

use request::VoucherifyRequest;
use voucher::create::VoucherCreateRequest;
use voucher::get::VoucherGetRequest;
use voucher::update::VoucherUpdateRequest;
use voucher::delete::VoucherDeleteRequest;
use voucher::list::VoucherListRequest;
use voucher::enable::VoucherEnableRequest;
use voucher::disable::VoucherDisableRequest;
use voucher::Voucher;

pub struct Voucherify {
    api_key: String,
    api_user: String,
}

impl Voucherify {
    pub fn new(key: &str, user: &str) -> Voucherify {
        Voucherify {
            api_key: key.to_string(),
            api_user: user.to_string(),
        }
    }

    /// Creates a voucher
    ///
    /// # Example
    ///
    /// ```
    /// use voucherify_rs::Voucherify;
    /// use voucherify_rs::voucher::{Voucher, VoucherType, DiscountType};
    ///
    /// let voucherify = Voucherify::new("c70a6f00-cf91-4756-9df5-47628850002b",
    ///                                  "3266b9f8-e246-4f79-bdf0-833929b1380c");
    ///
    /// let new_voucher = Voucher::new()
    ///     .voucher_type(VoucherType::DISCOUNT_VOUCHER)
    ///     .discount(DiscountType::AMOUNT, 20)
    ///     .build();
    ///
    /// let created_voucher = voucherify.voucher_create(new_voucher).send().unwrap();
    ///
    /// assert_eq!(created_voucher.discount.unwrap().amount_off, 20);
    /// ```
    pub fn voucher_create(&self, voucher: Voucher) -> VoucherCreateRequest {
        let new_request = VoucherifyRequest::new(&self.api_key, &self.api_user);
        VoucherCreateRequest::new(new_request, voucher)
    }

    /// Gets single voucher by id.
    ///
    /// # Example
    ///
    /// ```
    /// use voucherify_rs::Voucherify;
    /// use voucherify_rs::voucher::Voucher;
    ///
    /// let voucherify = Voucherify::new("c70a6f00-cf91-4756-9df5-47628850002b",
    ///                                  "3266b9f8-e246-4f79-bdf0-833929b1380c");
    ///
    /// let single_voucher: Voucher = voucherify.voucher_get("D1dsWQVE").send().unwrap();
    ///
    /// assert_eq!(single_voucher.code.unwrap(), "D1dsWQVE");
    /// ```
    pub fn voucher_get(&self, voucher_id: &str) -> VoucherGetRequest {
        let new_request = VoucherifyRequest::new(&self.api_key, &self.api_user);
        VoucherGetRequest::new(new_request, voucher_id)
    }

    /// Updates voucher by id.
    ///
    /// # Example
    ///
    /// ```
    /// use voucherify_rs::Voucherify;
    /// use voucherify_rs::voucher::Voucher;
    /// use voucherify_rs::utils::Metadata;
    ///
    /// let voucherify = Voucherify::new("c70a6f00-cf91-4756-9df5-47628850002b",
    ///                                  "3266b9f8-e246-4f79-bdf0-833929b1380c");
    ///
    /// let updated_metadata = Metadata::new()
    ///     .number("number", 32)
    ///     .string("is", "working")
    ///     .boolean("is_amazing", true)
    ///     .build();
    ///
    /// let updated_voucher = voucherify.voucher_update("D1dsWQVE")
    ///                                 .category("hello_world")
    ///                                 .active(true)
    ///                                 .metadata(updated_metadata)
    ///                                 // .gift_amount(1234)
    ///                                 .send().unwrap();
    ///
    /// assert_eq!(updated_voucher.category.unwrap(), "hello_world");
    /// ```
    pub fn voucher_update(&self, voucher_id: &str) -> VoucherUpdateRequest {
        let new_request = VoucherifyRequest::new(&self.api_key, &self.api_user);
        VoucherUpdateRequest::new(new_request, voucher_id)
    }

    /// Deletes voucher by id.
    ///
    /// # Example
    ///
    /// ```
    /// use voucherify_rs::Voucherify;
    /// use voucherify_rs::voucher::{Voucher, VoucherType, DiscountType};
    ///
    /// let voucherify = Voucherify::new("c70a6f00-cf91-4756-9df5-47628850002b",
    ///                                  "3266b9f8-e246-4f79-bdf0-833929b1380c");
    ///
    /// // First, we create a voucher
    ///
    /// let new_voucher = Voucher::new()
    ///     .voucher_type(VoucherType::DISCOUNT_VOUCHER)
    ///     .discount(DiscountType::AMOUNT, 34)
    ///     .build();
    ///
    /// let created_voucher = voucherify.voucher_create(new_voucher).send().unwrap();
    ///
    /// assert_eq!(created_voucher.discount.unwrap().amount_off, 34);
    ///
    ///
    /// // Then, we delete it
    ///
    /// let created_voucher_code = created_voucher.code.unwrap();
    ///
    /// let is_voucher_deleted: bool = voucherify.voucher_delete(created_voucher_code.as_str()).send().unwrap();
    ///
    /// assert_eq!(is_voucher_deleted, true);
    /// ```
    pub fn voucher_delete(&self, voucher_id: &str) -> VoucherDeleteRequest {
        let new_request = VoucherifyRequest::new(&self.api_key, &self.api_user);
        VoucherDeleteRequest::new(new_request, voucher_id)
    }

    /// Gets a list of vouchers.
    ///
    /// # Example
    ///
    /// ```
    /// use voucherify_rs::Voucherify;
    /// use voucherify_rs::voucher::Voucher;
    ///
    /// let voucherify = Voucherify::new("c70a6f00-cf91-4756-9df5-47628850002b",
    ///                                  "3266b9f8-e246-4f79-bdf0-833929b1380c");
    ///
    /// let voucher_list: Vec<Voucher> = voucherify.voucher_list().limit(19).page(1).send().unwrap();
    ///
    /// assert!(voucher_list.len() <= 19);
    /// ```
    pub fn voucher_list(&self) -> VoucherListRequest {
        let new_request = VoucherifyRequest::new(&self.api_key, &self.api_user);
        VoucherListRequest::new(new_request)
    }

    /// Enables voucher by id.
    ///
    /// # Example
    ///
    /// ```
    /// use voucherify_rs::Voucherify;
    /// use voucherify_rs::voucher::Voucher;
    ///
    /// let voucherify = Voucherify::new("c70a6f00-cf91-4756-9df5-47628850002b",
    ///                                  "3266b9f8-e246-4f79-bdf0-833929b1380c");
    ///
    /// let is_voucher_enabled: bool = voucherify.voucher_enable("D1dsWQVE").send().unwrap();
    ///
    /// assert_eq!(is_voucher_enabled, true);
    /// ```
    pub fn voucher_enable(&self, voucher_id: &str) -> VoucherEnableRequest {
        let new_request = VoucherifyRequest::new(&self.api_key, &self.api_user);
        VoucherEnableRequest::new(new_request, voucher_id)
    }

    /// Disables voucher by id.
    ///
    /// # Example
    ///
    /// ```
    /// use voucherify_rs::Voucherify;
    /// use voucherify_rs::voucher::Voucher;
    ///
    /// let voucherify = Voucherify::new("c70a6f00-cf91-4756-9df5-47628850002b",
    ///                                  "3266b9f8-e246-4f79-bdf0-833929b1380c");
    ///
    /// let is_voucher_disabled: bool = voucherify.voucher_disable("D1dsWQVE").send().unwrap();
    ///
    /// assert_eq!(is_voucher_disabled, true);
    /// ```
    pub fn voucher_disable(&self, voucher_id: &str) -> VoucherDisableRequest {
        let new_request = VoucherifyRequest::new(&self.api_key, &self.api_user);
        VoucherDisableRequest::new(new_request, voucher_id)
    }
}
