#[test]
fn home_dir_test() {
    let expected = dirs::home_dir();
    let resulted = simple_home_dir::home_dir();
    assert_eq!(expected, resulted)
}
