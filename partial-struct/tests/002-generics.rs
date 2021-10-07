#[macro_use]
extern crate partial_struct;

#[derive(PartialStruct)]
struct GenericConfig<T: std::fmt::Debug, V> {
  value_t: T,
  value_v: V,
}

#[test]
fn test_apply_partials() {
  let mut config = GenericConfig {
    value_t: 3.0,
    value_v: "foo",
  };

  let partial_config = PartialGenericConfig {
    value_t: None,
    value_v: Some("bar"),
  };

  config.apply_partials(partial_config);

  assert_eq!(config.value_t, 3.0);
  assert_eq!(config.value_v, "bar");
}
