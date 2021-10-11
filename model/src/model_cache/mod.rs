pub mod entity_cache;
pub mod field_cache;
pub mod model_cache;

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
