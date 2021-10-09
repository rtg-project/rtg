#[macro_use]
extern crate partial_struct;

#[derive(PartialStruct)]
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

#[test]
fn test_apply_partials() {
  // let mut config = Config::Basic {
  //   delay: 2,
  //   path: Some("/var/log/foo.log".to_owned()),
  //   percentage: 3.12,
  // };

  // let partial_config = PartialConfig::Basic {
  //   delay: None,
  //   path: Some("/tmp/bar.log".to_owned()),
  //   percentage: Some(42.24),
  // };

  // config.apply_partials(partial_config);

  // match config {
  //   Config::Basic {
  //     delay,
  //     path,
  //     percentage,
  //   } => {
  //     assert_eq!(delay, 2);
  //     assert_eq!(path, Some("/tmp/bar.log".to_owned()));
  //     assert_eq!(percentage, 42.24);
  //   }
  //   _ => panic!("Unexpected enum variant"),
  // }
}
