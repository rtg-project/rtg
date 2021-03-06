use super::ExplicitModel;
use schemars::{schema::RootSchema, schema_for};

pub fn generate_json_schema() -> RootSchema {
  return schema_for!(ExplicitModel);
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;
  use insta::assert_json_snapshot;

  #[test]
  fn json_schema_ok() {
    let schema = generate_json_schema();
    // The json schema is too long to check here, so we check it with insta
    assert_json_snapshot!(serde_json::to_string_pretty(&schema).unwrap());
  }
}
