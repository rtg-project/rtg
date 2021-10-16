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
    } => {
      let explicit_name = match name {
        Some(implicit_name) => implicit_name.to_owned(),
        None => match graphql_entity_type_name {
          Some(implicit_graphql_entity_type_name) => {
            entity::inflect_name_from_graphql_entity_type_name(implicit_graphql_entity_type_name)
          }
          None => match sql_table_name {
            Some(implicit_sql_table_name) => {
              entity::inflect_name_from_sql_table_name(implicit_sql_table_name)
            }
            None => return Err(ConversionError::EntityNameMissing),
          },
        },
      };

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
        None => return Err(ConversionError::EntityHasNoField(explicit_name.to_owned())),
        Some(field) => match field.as_ref() {
          ExplicitField::ScalarDatabaseColumn {
            graphql_order_by_asc,
            ..
          } => graphql_order_by_asc,
        },
      };

      Ok(ExplicitEntity::DatabaseTable {
        name: explicit_name.to_owned(),
        sql_schema_name: sql_schema_name
          .as_ref()
          .unwrap_or(&"public".to_owned())
          .to_owned(),
        sql_table_name: sql_table_name
          .as_ref()
          .unwrap_or(&entity::inflect_sql_table_name_from_name(
            explicit_name.as_ref(),
          ))
          .to_owned(),
        graphql_entity_type_name: graphql_entity_type_name
          .as_ref()
          .unwrap_or(&entity::inflect_graphql_entity_type_name_from_name(
            explicit_name.as_ref(),
          ))
          .to_owned(),
        graphql_filter_type_name: graphql_filter_type_name
          .as_ref()
          .unwrap_or(&entity::inflect_graphql_filter_type_name_from_name(
            explicit_name.as_ref(),
          ))
          .to_owned(),
        graphql_get_single_operation_name: graphql_get_single_operation_name
          .as_ref()
          .unwrap_or(
            &entity::inflect_graphql_get_single_operation_name_from_name(explicit_name.as_ref()),
          )
          .to_owned(),
        graphql_get_list_operation_name: graphql_get_list_operation_name
          .as_ref()
          .unwrap_or(&entity::inflect_graphql_get_list_operation_name_from_name(
            explicit_name.as_ref(),
          ))
          .to_owned(),
        graphql_get_connection_operation_name: graphql_get_connection_operation_name
          .as_ref()
          .unwrap_or(
            &entity::inflect_graphql_get_connection_operation_name_from_name(
              explicit_name.as_ref(),
            ),
          )
          .to_owned(),
        graphql_default_order_by: graphql_default_order_by
          .as_ref()
          .unwrap_or(default_graphql_default_order_by)
          .to_owned(),
        graphql_default_first: graphql_default_first.unwrap_or(10),
        graphql_default_offset: graphql_default_offset.unwrap_or(0),
        fields: fields,
      })
    }
  }
}
