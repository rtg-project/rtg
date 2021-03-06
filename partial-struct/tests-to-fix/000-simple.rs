#[macro_use]
extern crate partial_struct;

#[derive(PartialStruct)]
struct Config {
  delay: Option<u32>,
  path: String,
  percentage: f32,
}

#[test]
fn test_apply_partials() {
  let mut config = Config {
    delay: Some(2),
    path: "/var/log/foo.log".to_owned(),
    percentage: 3.12,
  };

  let partial_config = ConfigPartial {
    delay: None,
    path: Some("/tmp/bar.log".to_owned()),
    percentage: Some(42.24),
  };

  config.apply_partials(partial_config);

  assert_eq!(config.delay, None);
  assert_eq!(config.path, "/tmp/bar.log");
  assert_eq!(config.percentage, 42.24);
}
