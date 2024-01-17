#[test]
fn home_dir_test() {
    let expected = dirs::home_dir().unwrap();
    let resulted = simple_home_dir::home_dir().unwrap();
    assert_eq!(resulted, expected)
}
