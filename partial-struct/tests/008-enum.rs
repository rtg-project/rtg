#[macro_use]
extern crate partial_struct;
use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, Serialize)]
#[derive(PartialStruct)]
// #[serde(tag = "type", rename_all = "camelCase")]
#[partial(name = "ConfigOption", completion = "complete_config")]
#[partial_attribute(derive(Debug))]
// #[partial_attribute(serde(tag = "type", rename_all = "camelCase"))]
enum Config {
  // #[serde(rename_all = "camelCase")]
  #[partial(skip)]
  Simple {
    #[partial(name = "lol")]
    foo: Option<u32>,
    bar: String,
    percentage: f32,
  },
  // #[serde(rename_all = "camelCase")]
  #[partial(name = "Basique")]
  Basic {
    #[partial(require)]
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
