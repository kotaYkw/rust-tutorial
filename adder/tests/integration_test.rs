extern crate adder;

mod common;

#[test]
fn it_ads_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
