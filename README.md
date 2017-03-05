# voucherify-rust-sdk

## API
### Setup

Import voucherify-rs crate

``` rust
extern crate voucherify_rs;
```

Create voucherify api object

``` rust
let voucherify = Voucherify::new("<YOUR_APP_ID_GOES_HERE>",
                                 "<YOUR_SECRET_KEY_GOES_HERE>");
```

### Vouchers API
Methods are provided:
- [Create Voucher](#create-voucher)
- [Get Voucher](#get-voucher)
- [Update Voucher](#update-voucher)
- [Delete Voucher](#delete-voucher)
- [List Vouchers](#list-vouchers)
- [Enable Voucher](#enable-voucher)
- [Disable Voucher](#disable-voucher)
- [Import Vouchers](#import-vouchers)

#### [Create Voucher]

``` rust
let new_voucher = Voucher::new()
    .voucher_type(VoucherType::DISCOUNT_VOUCHER)
    .discount(DiscountType::AMOUNT, 20)
    .build();

let returned_voucher = voucherify.voucher_create(new_voucher).send().unwrap();
```

#### [Get Voucher]

``` rust
let single_voucher: Voucher = voucherify.voucher_get("yv5k3rFu").send().unwrap();
```
[Create Voucher]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#create-voucher
[Get Voucher]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#vouchers-get
[Update Voucher]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#update-voucher
[Delete Voucher]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#delete-voucher
[List Vouchers]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#list-vouchers
[Enable Voucher]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#enable-voucher
[Disable Voucher]: https://docs.voucherify.io/reference?utm_source=github&utm_medium=sdk&utm_campaign=acq#disable-voucher
