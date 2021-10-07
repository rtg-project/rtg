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
  let partial_config = PartialConfig::empty();

  assert_eq!(partial_config.delay, None);
  assert_eq!(partial_config.path, None);
  assert_eq!(partial_config.percentage, None);
}
