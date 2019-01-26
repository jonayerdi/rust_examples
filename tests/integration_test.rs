// Cannot do integration tests for binary crates!!
use rust_examples;
#[test]
fn integration1() {
    assert_eq!(rust_examples::add(1, 2), 3);
}
