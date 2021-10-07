struct Config {
  delay: Option<u32>,
  path: String,
  percentage: f32,
}

struct PartialConfig {
  delay: Option<u32>,
  path: Option<String>,
  percentage: Option<f32>,
}

impl Config {
  pub fn apply_partials(&mut self, partial_struct: PartialConfig) {
    if let Some(field) = partial_struct.delay {
      self.delay = Some(field);
    }

    if let Some(field) = partial_struct.path {
      self.path = field;
    }

    if let Some(field) = partial_struct.percentage {
      self.percentage = field;
    }
  }
}

#[test]
fn test_apply_partials() {
  let mut config = Config {
    delay: Some(2),
    path: "/var/log/foo.log".to_owned(),
    percentage: 3.12,
  };

  let partial_config = PartialConfig {
    delay: None,
    path: Some("/tmp/bar.log".to_owned()),
    percentage: Some(42.24),
  };

  config.apply_partials(partial_config);

  assert_eq!(config.delay, Some(2));
  assert_eq!(config.path, "/tmp/bar.log");
  assert_eq!(config.percentage, 42.24);
}
