use super::field_cache::FieldCache;
use crate::explicit_model::entity::ExplicitEntity;
use crate::explicit_model::field::ExplicitField;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum EntityCache {
  #[serde(rename_all = "camelCase")]
  DatabaseTable {
    entity: Rc<ExplicitEntity>,
    fields_by_sql_column_name: FxHashMap<String, Rc<FieldCache>>,
    fields_by_graphql_field_name: FxHashMap<String, Rc<FieldCache>>,
  },
}

impl EntityCache {
  // Another associated function, taking two arguments:
  pub fn new(entity: Rc<ExplicitEntity>) -> EntityCache {
    match &*entity {
      ExplicitEntity::DatabaseTable { fields, .. } => {
        let mut fields_by_sql_column_name = FxHashMap::default();
        let mut fields_by_graphql_field_name = FxHashMap::default();
        for field in fields.iter() {
          match &**field {
            ExplicitField::ScalarDatabaseColumn {
              sql_column_name,
              graphql_field_name,
              ..
            } => {
              let field_cache = Rc::new(FieldCache::new(Rc::clone(&field)));
              fields_by_sql_column_name.insert(sql_column_name.clone(), Rc::clone(&field_cache));
              fields_by_graphql_field_name
                .insert(graphql_field_name.clone(), Rc::clone(&field_cache));
            }
          }
        }
        return EntityCache::DatabaseTable {
          entity: Rc::clone(&entity),
          fields_by_sql_column_name,
          fields_by_graphql_field_name,
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
    let value = Rc::new(ExplicitEntity::DatabaseTable {
      name: "person".to_owned(),
      sql_schema_name: "public".to_owned(),
      sql_table_name: "person_table".to_owned(),
      graphql_entity_type_name: "Person".to_owned(),
      graphql_filter_type_name: "PersonWhereFilter".to_owned(),
      graphql_get_single_operation_name: "person".to_owned(),
      graphql_get_list_operation_name: "persons".to_owned(),
      graphql_get_connection_operation_name: "personConnection".to_owned(),
      graphql_default_order_by: "id_ASC".to_owned(),
      graphql_default_first: 10,
      graphql_default_offset: 0,
      fields: vec![
        Rc::new(ExplicitField::ScalarDatabaseColumn {
          name: "id".to_owned(),
          nullable: false,
          sql_type: sql_type::Type::Text,
          sql_column_name: "id_col".to_owned(),
          graphql_enabled: true,
          graphql_field_name: "id".to_owned(),
          graphql_type_name: "String".to_owned(),
          graphql_order_by_asc: "id_ASC".to_owned(),
          graphql_order_by_desc: "id_DESC".to_owned(),
        }),
        Rc::new(ExplicitField::ScalarDatabaseColumn {
          name: "drone".to_owned(),
          nullable: false,
          sql_type: sql_type::Type::Text,
          sql_column_name: "drone_col".to_owned(),
          graphql_enabled: true,
          graphql_field_name: "drone".to_owned(),
          graphql_type_name: "String".to_owned(),
          graphql_order_by_asc: "drone_ASC".to_owned(),
          graphql_order_by_desc: "drone_DESC".to_owned(),
        }),
      ],
    });

    assert_debug_snapshot!(EntityCache::new(Rc::clone(&value)));
    assert_json_snapshot!(
      serde_json::to_string_pretty(&EntityCache::new(Rc::clone(&value))).unwrap()
    );
  }
}
