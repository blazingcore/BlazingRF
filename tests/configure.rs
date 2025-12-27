#[test]
fn config_test() {
    let mut sys = blast::config::System::init();
    sys.backup_config("backups").unwrap();
    let result = blast::Blast::new(Some(8088));
    assert!(!result.config.get_secret().is_empty());
    assert_eq!(result.config.get_port(), 8088);
    sys.server.set_port(8888);
    sys.save_config().unwrap();
    let new_sys = blast::config::System::init();
    assert_eq!(new_sys.server.get_port(), 8888);
    sys.restore_from_backup("backups").unwrap();
}
