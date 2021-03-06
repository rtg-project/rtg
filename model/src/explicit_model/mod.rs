pub mod entity;
pub mod field;
pub mod json_schema;
pub mod model;
pub mod sql_type;

pub use entity::ExplicitEntity;
pub use field::ExplicitField;
pub use json_schema::generate_json_schema;
pub use model::ExplicitModel;
pub use sql_type::Type;

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
