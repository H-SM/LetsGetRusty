use testing;

mod common;
//just lok over the file naming and the folder structure (we cant change it for these common modules)

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, testing::add(2,2));
}