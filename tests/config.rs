use bakong_khqr::config::BakongConfig;

#[test]
fn test_sandbox_config() {
    let config = BakongConfig::sandbox("token");
    assert_eq!(config.token, "token");
    assert!(config.is_sandbox());
}

#[test]
fn test_production_config() {
    let config = BakongConfig::production("token");
    assert!(!config.is_sandbox());
}

#[test]
fn test_custom_base_url() {
    let config = BakongConfig::sandbox("token").with_base_url("http://localhost:8080");

    assert_eq!(config.base_url, Some("http://localhost:8080".to_string()));
}
