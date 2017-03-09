# voucherify-rust-sdk

## Setup

Add crate to your Cargo.toml
``` toml
[dependencies]
voucherify_rs = "0.1.1"
```

Import voucherify-rs crate

``` rust
extern crate voucherify_rs;
```

Create voucherify api object

``` rust
let voucherify = Voucherify::new("<YOUR_APP_ID_GOES_HERE>",
                                 "<YOUR_SECRET_KEY_GOES_HERE>");
```

## Vouchers API

Provided methods:
- [Create Voucher](#create-voucher)
- [Get Voucher](#get-voucher)
- [Update Voucher](#update-voucher)
- [Delete Voucher](#delete-voucher)
- [List Vouchers](#list-vouchers)
- [Enable Voucher](#enable-voucher)
- [Disable Voucher](#disable-voucher)

### [Create Voucher]

``` rust
let new_voucher = Voucher::new()
    .voucher_type(VoucherType::DISCOUNT_VOUCHER)
    .discount(DiscountType::AMOUNT, 20)
    .build();

let created_voucher = voucherify.voucher_create(new_voucher).send().unwrap();
```

### [Get Voucher]

``` rust
let single_voucher: Voucher = voucherify.voucher_get("D1dsWQVE").send().unwrap();
```

### [Update Voucher]

``` rust
let updated_metadata = Metadata::new()
    .number("number", 32)
    .string("is", "working")
    .boolean("is_amazing", true)
    .build();

let updated_voucher = voucherify.voucher_update("D1dsWQVE")
                                .category("hello_world")
                                .active(true)
                                .metadata(updated_metadata)
                                // .gift_amount(1234)
                                .send().unwrap();
```

### [Delete Voucher]

``` rust
let was_voucher_deleted: bool = voucherify.voucher_delete(created_voucher_code.as_str()).send().unwrap();
```

### [List Vouchers]

``` rust
let voucher_list: Vec<Voucher> = voucherify.voucher_list().limit(19).page(1).send().unwrap();
```

### [Enable Voucher]

``` rust
let was_voucher_enabled: bool = voucherify.voucher_enable("D1dsWQVE").send().unwrap();
```

### [Disable Voucher]

``` rust
let was_voucher_disabled: bool = voucherify.voucher_disable("D1dsWQVE").send().unwrap();
```

## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

[Create Voucher]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#create-voucher
[Get Voucher]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#vouchers-get
[Update Voucher]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#update-voucher
[Delete Voucher]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#delete-voucher
[List Vouchers]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#list-vouchers
[Enable Voucher]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#enable-voucher
[Disable Voucher]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#disable-voucher
