use rtg_model::field::Field;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum FieldCache {
  #[serde(rename_all = "camelCase")]
  ScalarDatabaseColumn { field: Rc<Field> },
}

impl FieldCache {
  // Another associated function, taking two arguments:
  pub fn new(field: Rc<Field>) -> FieldCache {
    match *field {
      Field::ScalarDatabaseColumn { .. } => {
        return FieldCache::ScalarDatabaseColumn {
          field: Rc::clone(&field),
        };
      }
    };
  }
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;
  use rtg_model::sql_type;

  #[test]
  fn constructor() {
    let value = Rc::new(Field::ScalarDatabaseColumn {
      name: "id".to_string(),
      sql_type: sql_type::Type::Text,
      sql_column_name: "id_col".to_string(),
      graphql_field_name: "id".to_string(),
      graphql_type_name: "String".to_string(),
      graphql_order_by_asc: "id_ASC".to_string(),
      graphql_order_by_desc: "id_DESC".to_string(),
    });

    insta::assert_debug_snapshot!(FieldCache::new(Rc::clone(&value)));
    insta::assert_debug_snapshot!(serde_json::to_string_pretty(&FieldCache::new(Rc::clone(
      &value
    )))
    .unwrap());
  }
}
