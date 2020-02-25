use controlling_how_tests_are_run::add_two;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}