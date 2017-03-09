extern crate voucherify_rs;

use voucherify_rs::Voucherify;
use voucherify_rs::voucher::{Voucher, VoucherType, DiscountType};
use voucherify_rs::utils::Metadata;

fn main() {

    //
    // Setup voucherify api
    //
    let voucherify = Voucherify::new("c70a6f00-cf91-4756-9df5-47628850002b",
                                     "3266b9f8-e246-4f79-bdf0-833929b1380c");


    //
    // Crate a voucher object
    //
    let new_voucher = Voucher::new()
        .voucher_type(VoucherType::DISCOUNT_VOUCHER)
        .discount(DiscountType::AMOUNT, 20)
        .build();

    // Request voucher to be created
    let created_voucher: Voucher = voucherify.voucher_create(new_voucher).send().unwrap();
    println!("{:?}", created_voucher);

    // Voucherify API returns the voucher it just created
    let created_voucher_code = created_voucher.code.unwrap();


    //
    // Get voucher
    //
    let single_voucher: Voucher = voucherify.voucher_get(created_voucher_code.as_str()).send().unwrap();
    println!("{:?}", single_voucher);


    //
    // Update voucher
    //
    // Create metadata
    let updated_metadata = Metadata::new()
        .number("number", 32)
        .string("is", "working")
        .boolean("is_amazing", true)
        .build();

    // Request voucher to be updated
    let updated_voucher = voucherify.voucher_update(created_voucher_code.as_str())
                                    .category("hello_world")
                                    .active(true)
                                    .metadata(updated_metadata)
                                    .send().unwrap();
    println!("{:?}", updated_voucher);


    //
    // Disable voucher
    //
    let is_voucher_disabled: bool = voucherify.voucher_disable(created_voucher_code.as_str()).send().unwrap();
    println!("Is voucher disabled? {}", is_voucher_disabled);


    //
    // List vouchers
    //
    let voucher_list: Vec<Voucher> = voucherify.voucher_list().limit(3).page(1).send().unwrap();
    println!("{:?}", voucher_list);


    //
    // Enable voucher
    //
    let is_voucher_enabled: bool = voucherify.voucher_enable(created_voucher_code.as_str()).send().unwrap();
    println!("Is voucher enabled? {}", is_voucher_enabled);

    //
    // Delete voucher
    //
    let is_voucher_deleted: bool = voucherify.voucher_delete(created_voucher_code.as_str()).send().unwrap();
    println!("Is voucher deleted? {}", is_voucher_deleted);

}
