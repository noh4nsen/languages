use test_organization as to;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, to::add_two(2));
}