/* Integration tests */
// to test the whole of your lib (even if each mod can pass the unit test, that doesn't mean it can as well when integrated altogether)

use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
