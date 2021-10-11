use crate::explicit_model::field::ExplicitField;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum FieldCache {
  #[serde(rename_all = "camelCase")]
  ScalarDatabaseColumn { field: Rc<ExplicitField> },
}

impl FieldCache {
  // Another associated function, taking two arguments:
  pub fn new(field: Rc<ExplicitField>) -> FieldCache {
    match *field {
      ExplicitField::ScalarDatabaseColumn { .. } => {
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
  use crate::explicit_model::sql_type;
  use insta::{assert_debug_snapshot, assert_json_snapshot};

  #[test]
  fn constructor() {
    let value = Rc::new(ExplicitField::ScalarDatabaseColumn {
      name: "id".to_string(),
      sql_type: sql_type::Type::Text,
      sql_column_name: "id_col".to_string(),
      graphql_field_name: "id".to_string(),
      graphql_type_name: "String".to_string(),
      graphql_order_by_asc: "id_ASC".to_string(),
      graphql_order_by_desc: "id_DESC".to_string(),
    });

    assert_debug_snapshot!(FieldCache::new(Rc::clone(&value)));
    assert_json_snapshot!(
      serde_json::to_string_pretty(&FieldCache::new(Rc::clone(&value))).unwrap()
    );
  }
}
