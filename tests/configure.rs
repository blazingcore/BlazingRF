#[test]
fn config_init_test() {
    let result = blast::Blast::new(Some(8088));
    assert!(!result.config.get_secret().is_empty());
    assert_eq!(result.config.get_port(), 8088);
}
