use super::conversion_error::ConversionError;
use super::convert_field::convert_field;
use crate::explicit_model::{ExplicitEntity, ExplicitField};
use crate::implicit_model::ImplicitEntity;
use crate::inflection::entity;

use std::rc::Rc;

pub fn convert_entity(implicit_entity: &ImplicitEntity) -> Result<ExplicitEntity, ConversionError> {
  match &*implicit_entity {
    ImplicitEntity::DatabaseTable {
      name,
      sql_schema_name,
      sql_table_name,
      graphql_entity_type_name,
      graphql_filter_type_name,
      graphql_get_single_operation_name,
      graphql_get_list_operation_name,
      graphql_get_connection_operation_name,
      graphql_default_order_by,
      graphql_default_first,
      graphql_default_offset,
      fields,
    } => match name {
      Some(name) => {
        let fields = match fields {
          Some(fields) => fields
            .iter()
            .map(|field| match convert_field(field) {
              Ok(field) => Ok(Rc::new(field)),
              Err(err) => Err(err),
            })
            .collect::<Result<Vec<Rc<ExplicitField>>, ConversionError>>()?,
          None => vec![],
        };
        let default_graphql_default_order_by = match fields.first() {
          None => return Err(ConversionError::EntityHasNoField(name.to_string())),
          Some(field) => match field.as_ref() {
            ExplicitField::ScalarDatabaseColumn {
              graphql_order_by_asc,
              ..
            } => graphql_order_by_asc,
            _ => return Err(ConversionError::EntityHasNoField(name.to_string())),
          },
        };

        Ok(ExplicitEntity::DatabaseTable {
          name: name.to_string(),
          sql_schema_name: sql_schema_name.as_ref().unwrap_or(name).to_string(),
          sql_table_name: sql_table_name
            .as_ref()
            .unwrap_or(&entity::inflect_sql_table_name_from_name(name))
            .to_string(),
          graphql_entity_type_name: graphql_entity_type_name
            .as_ref()
            .unwrap_or(&entity::inflect_graphql_entity_type_name_from_name(name))
            .to_string(),
          graphql_filter_type_name: graphql_filter_type_name
            .as_ref()
            .unwrap_or(&entity::inflect_graphql_filter_type_name_from_name(name))
            .to_string(),
          graphql_get_single_operation_name: graphql_get_single_operation_name
            .as_ref()
            .unwrap_or(&entity::inflect_graphql_get_single_operation_name_from_name(name))
            .to_string(),
          graphql_get_list_operation_name: graphql_get_list_operation_name
            .as_ref()
            .unwrap_or(&entity::inflect_graphql_get_list_operation_name_from_name(
              name,
            ))
            .to_string(),
          graphql_get_connection_operation_name: graphql_get_connection_operation_name
            .as_ref()
            .unwrap_or(&entity::inflect_graphql_get_connection_operation_name_from_name(name))
            .to_string(),
          graphql_default_order_by: graphql_default_order_by
            .as_ref()
            .unwrap_or(default_graphql_default_order_by)
            .to_string(),
          graphql_default_first: graphql_default_first.unwrap_or(10),
          graphql_default_offset: graphql_default_offset.unwrap_or(0),
          fields: fields,
        })
      }
      None => return Err(ConversionError::EntityNameMissing),
    },
  }
}
