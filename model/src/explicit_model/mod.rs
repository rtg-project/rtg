pub mod entity;
pub mod field;
pub mod json_schema;
pub mod model;
pub mod sql_type;

// Tests
#[cfg(test)]
mod tests {

  use similar_asserts::assert_eq;
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn it_works2() {
    assert_eq!(3 + 2, 5);
  }
}
