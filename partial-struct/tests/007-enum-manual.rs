#[macro_use]
extern crate partial_struct;

// #[derive(PartialStruct)]
enum Config {
  Simple {
    foo: Option<u32>,
    bar: String,
    percentage: f32,
  },
  Basic {
    delay: u32,
    path: Option<String>,
    percentage: f32,
  },
}

enum PartialConfig {
  Simple {
    foo: Option<u32>,
    bar: Option<String>,
    percentage: Option<f32>,
  },
  Basic {
    delay: Option<u32>,
    path: Option<String>,
    percentage: Option<f32>,
  },
}

impl Config {
  pub fn apply_partials(&mut self, partial_struct: PartialConfig) {
    match partial_struct {
      PartialConfig::Simple {
        foo: partial_foo,
        bar: partial_bar,
        percentage: partial_percentage,
      } => {
        if let Config::Simple {
          foo: ref mut self_foo,
          bar: ref mut self_bar,
          percentage: ref mut self_percentage,
        } = self
        {
          if let Some(field) = partial_foo {
            *self_foo = Some(field);
          }
          if let Some(field) = partial_bar {
            *self_bar = field;
          }
          if let Some(field) = partial_percentage {
            *self_percentage = field;
          }
          return;
        } else {
          panic!("Unexpected enum variant");
        }
      }
      PartialConfig::Basic {
        delay: partial_delay,
        path: partial_path,
        percentage: partial_percentage,
      } => {
        if let Config::Basic {
          delay: ref mut self_delay,
          path: ref mut self_path,
          percentage: ref mut self_percentage,
        } = self
        {
          if let Some(field) = partial_delay {
            *self_delay = field;
          }
          if let Some(field) = partial_path {
            *self_path = Some(field);
          }
          if let Some(field) = partial_percentage {
            *self_percentage = field;
          }
          return;
        } else {
          panic!("Unexpected enum variant");
        }
      }
    };
  }
}

#[test]
fn test_apply_partials() {
  let mut config = Config::Basic {
    delay: 2,
    path: Some("/var/log/foo.log".to_owned()),
    percentage: 3.12,
  };

  let partial_config = PartialConfig::Basic {
    delay: None,
    path: Some("/tmp/bar.log".to_owned()),
    percentage: Some(42.24),
  };

  config.apply_partials(partial_config);

  match config {
    Config::Basic {
      delay,
      path,
      percentage,
    } => {
      assert_eq!(delay, 2);
      assert_eq!(path, Some("/tmp/bar.log".to_owned()));
      assert_eq!(percentage, 42.24);
    }
    _ => panic!("Unexpected enum variant"),
  }
}
