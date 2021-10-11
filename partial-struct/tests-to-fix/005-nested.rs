#[macro_use]
extern crate partial_struct;

#[derive(PartialStruct)]
// #[partial_nested_original(LogConfig)]
// #[partial_nested_generated(PartialLogConfig)]
struct Config {
  timeout: Option<u32>,
  #[partial(nested_type = "LogConfigPartial")]
  log_config: LogConfig,
}

#[derive(PartialStruct)]
struct LogConfig {
  log_file: String,
  log_level: usize,
}

#[test]
fn test_apply_partials() {
  let mut config = Config {
    timeout: Some(2),
    log_config: LogConfig {
      log_file: "/var/log/foobar.log".to_owned(),
      log_level: 3,
    },
  };

  let partial_config = ConfigPartial {
    timeout: None,
    log_config: Some(LogConfigPartial {
      log_file: Some("/tmp/bar.log".to_owned()),
      log_level: None,
    }),
  };

  config.apply_partials(partial_config);

  assert_eq!(config.timeout, None);
  assert_eq!(config.log_config.log_file, "/tmp/bar.log");
  assert_eq!(config.log_config.log_level, 3);
}
