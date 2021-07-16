extern crate voucherify_rs;

use voucherify_rs::Voucherify;
use voucherify_rs::async_action::AsyncAction;
use voucherify_rs::async_action::AsyncActionList;

fn main() {

    //
    // Setup voucherify api
    //
    let voucherify = Voucherify::new("c70a6f00-cf91-4756-9df5-47628850002b",
                                     "3266b9f8-e246-4f79-bdf0-833929b1380c");

    //
    // List Async Actions
    //
    let async_actions: AsyncActionList = voucherify.async_action_list().limit(3).end_date("2021-07-16T16:10:28Z").send().unwrap();
    println!("Async Actions: {:?}", async_actions);

    //
    // Get Async Action
    //
    let async_action: AsyncAction = voucherify.async_action_get("id").send().unwrap();
    println!("Fetched Async Action: {:?}", async_action);

}
