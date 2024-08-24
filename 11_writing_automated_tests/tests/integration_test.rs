// cmd: cargo test (runs all tests, unit tests first, if unit tests failm it does not run the integration tests)
// cmd: cargo test --test integration_test

use writing_automated_tests::add_two; // identifiers cannot use start with numbers - 11_
mod common;

#[test]
fn it_adds_two() {
    common::setup(); // <-- this is how to use a function setup from another file
    assert_eq!(4, add_two(2));
}