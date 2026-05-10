use cli_utils::config::{Logging, LogLevel, LogOutput};

#[test]
fn test_config_setter() {
    let mut config = Logging::new();
    config.setter(true, LogLevel::Debug, LogOutput::Stderr);
    assert!(matches!(&config.level, LogLevel::Debug));
    assert!(matches!(&config.destination, LogOutput::Stderr));
}
