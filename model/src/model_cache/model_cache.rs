use super::entity_cache::EntityCache;
use crate::explicit_model::entity::ExplicitEntity;
use crate::explicit_model::model::ExplicitModel;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OperationKind {
  GetSingle,
  GetConnection,
  GetList,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntityOperationCache {
  pub entity_cache: Rc<EntityCache>,
  pub operation_kind: OperationKind,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "version", rename_all = "camelCase")]
pub enum ModelCache {
  #[serde(rename_all = "camelCase")]
  V1 {
    model: Rc<ExplicitModel>,
    entities_by_operation_name: FxHashMap<String, EntityOperationCache>,
  },
}

impl ModelCache {
  // Another associated function, taking two arguments:
  pub fn new(model: Rc<ExplicitModel>) -> ModelCache {
    match &*model {
      ExplicitModel::V1 { entities, .. } => {
        let mut entities_by_operation_name = FxHashMap::default();
        for entity in entities.iter() {
          match &**entity {
            ExplicitEntity::DatabaseTable {
              graphql_get_single_operation_name,
              graphql_get_list_operation_name,
              graphql_get_connection_operation_name,
              ..
            } => {
              let entity_cache = Rc::new(EntityCache::new(Rc::clone(&entity)));
              entities_by_operation_name.insert(
                graphql_get_single_operation_name.clone(),
                EntityOperationCache {
                  entity_cache: Rc::clone(&entity_cache),
                  operation_kind: OperationKind::GetSingle,
                },
              );
              entities_by_operation_name.insert(
                graphql_get_list_operation_name.clone(),
                EntityOperationCache {
                  entity_cache: Rc::clone(&entity_cache),
                  operation_kind: OperationKind::GetList,
                },
              );
              entities_by_operation_name.insert(
                graphql_get_connection_operation_name.clone(),
                EntityOperationCache {
                  entity_cache: Rc::clone(&entity_cache),
                  operation_kind: OperationKind::GetConnection,
                },
              );
            }
          }
        }
        return ModelCache::V1 {
          model: Rc::clone(&model),
          entities_by_operation_name,
        };
      }
    };
  }
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;
  use crate::explicit_model::entity::ExplicitEntity;
  use crate::explicit_model::field::ExplicitField;
  use crate::explicit_model::sql_type;
  use insta::{assert_debug_snapshot, assert_json_snapshot};
  use similar_asserts::assert_eq;

  #[test]
  fn constructor() {
    let value = Rc::new(ExplicitModel::V1 {
      entities: vec![Rc::new(ExplicitEntity::DatabaseTable {
        name: "person".to_string(),
        sql_schema_name: "public".to_string(),
        sql_table_name: "person_table".to_string(),
        graphql_entity_type_name: "Person".to_string(),
        graphql_filter_type_name: "PersonWhereFilter".to_string(),
        graphql_get_single_operation_name: "person".to_string(),
        graphql_get_list_operation_name: "persons".to_string(),
        graphql_get_connection_operation_name: "personConnection".to_string(),
        graphql_default_order_by: "id_ASC".to_string(),
        graphql_default_first: 10,
        graphql_default_offset: 0,
        fields: vec![
          Rc::new(ExplicitField::ScalarDatabaseColumn {
            name: "id".to_string(),
            sql_type: sql_type::Type::Text,
            sql_column_name: "id_col".to_string(),
            graphql_field_name: "id".to_string(),
            graphql_type_name: "String".to_string(),
            graphql_order_by_asc: "id_ASC".to_string(),
            graphql_order_by_desc: "id_DESC".to_string(),
          }),
          Rc::new(ExplicitField::ScalarDatabaseColumn {
            name: "drone".to_string(),
            sql_type: sql_type::Type::Text,
            sql_column_name: "drone_col".to_string(),
            graphql_field_name: "drone".to_string(),
            graphql_type_name: "String".to_string(),
            graphql_order_by_asc: "drone_ASC".to_string(),
            graphql_order_by_desc: "drone_DESC".to_string(),
          }),
        ],
      })],
    });

    assert_debug_snapshot!(ModelCache::new(Rc::clone(&value)));
    assert_json_snapshot!(
      serde_json::to_string_pretty(&ModelCache::new(Rc::clone(&value))).unwrap()
    );
  }

  #[test]
  fn serialize_entity_operation_cache() {
    let value = EntityOperationCache {
      entity_cache: Rc::new(EntityCache::new(Rc::new(ExplicitEntity::DatabaseTable {
        name: "person".to_string(),
        sql_schema_name: "public".to_string(),
        sql_table_name: "person_table".to_string(),
        graphql_entity_type_name: "Person".to_string(),
        graphql_filter_type_name: "PersonWhereFilter".to_string(),
        graphql_get_single_operation_name: "person".to_string(),
        graphql_get_list_operation_name: "persons".to_string(),
        graphql_get_connection_operation_name: "personConnection".to_string(),
        graphql_default_order_by: "id_ASC".to_string(),
        graphql_default_first: 10,
        graphql_default_offset: 0,
        fields: vec![],
      }))),
      operation_kind: OperationKind::GetSingle,
    };

    match serde_json::to_string_pretty(&value) {
      Ok(string) => {
        assert_eq!(
          string,
          r#"{
  "entityCache": {
    "type": "databaseTable",
    "entity": {
      "type": "databaseTable",
      "name": "person",
      "sqlSchemaName": "public",
      "sqlTableName": "person_table",
      "graphqlEntityTypeName": "Person",
      "graphqlFilterTypeName": "PersonWhereFilter",
      "graphqlGetSingleOperationName": "person",
      "graphqlGetListOperationName": "persons",
      "graphqlGetConnectionOperationName": "personConnection",
      "graphqlDefaultOrderBy": "id_ASC",
      "graphqlDefaultFirst": 10,
      "graphqlDefaultOffset": 0,
      "fields": []
    },
    "fieldsBySqlColumnName": {},
    "fieldsByGraphqlFieldName": {}
  },
  "operationKind": "getSingle"
}"#
        );
      }
      Err(e) => panic!("{}", e),
    }
  }
}
