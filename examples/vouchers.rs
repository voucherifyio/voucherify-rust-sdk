extern crate voucherify_rs;

use voucherify_rs::Voucherify;
use voucherify_rs::voucher::Voucher;

fn main() {
    let voucherify = Voucherify::new(
            "k1234",
            "u4321",
        );

    let hello: Vec<Voucher> = voucherify.voucher_list().limit(2).page(1).send();
    println!("{:?}", hello);
}
