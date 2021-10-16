use super::conversion_error::ConversionError;
use crate::explicit_model::ExplicitField;
use crate::implicit_model::ImplicitField;
use crate::inflection::field;

pub fn convert_field(implicit_field: &ImplicitField) -> Result<ExplicitField, ConversionError> {
  match &*implicit_field {
    ImplicitField::ScalarDatabaseColumn {
      name,
      nullable,
      sql_type,
      sql_column_name,
      graphql_field_name,
      graphql_type_name,
      graphql_order_by_asc,
      graphql_order_by_desc,
    } => {
      let explicit_name = match name {
        Some(implicit_name) => implicit_name.to_string(),
        None => match graphql_field_name {
          Some(implicit_graphql_field_name) => {
            field::inflect_name_from_graphql_field_name(implicit_graphql_field_name)
          }
          None => match sql_column_name {
            Some(implicit_sql_column_name) => {
              field::inflect_name_from_sql_column_name(implicit_sql_column_name)
            }
            None => return Err(ConversionError::FieldNameMissing),
          },
        },
      };
      match sql_type {
        Some(sql_type) => Ok(ExplicitField::ScalarDatabaseColumn {
          name: explicit_name.to_string(),
          nullable: nullable.unwrap_or(true),
          sql_type: (*sql_type).clone(),
          sql_column_name: sql_column_name
            .as_ref()
            .unwrap_or(&field::inflect_sql_column_name_from_name(&explicit_name))
            .to_string(),
          graphql_field_name: graphql_field_name
            .as_ref()
            .unwrap_or(&field::inflect_graphql_field_name_from_name(&explicit_name))
            .to_string(),
          graphql_type_name: graphql_type_name.as_ref().unwrap().to_string(),
          graphql_order_by_asc: graphql_order_by_asc
            .as_ref()
            .unwrap_or(&field::inflect_graphql_order_by_asc_from_name(
              &explicit_name,
            ))
            .to_string(),
          graphql_order_by_desc: graphql_order_by_desc
            .as_ref()
            .unwrap_or(&field::inflect_graphql_order_by_desc_from_name(
              &explicit_name,
            ))
            .to_string(),
        }),
        None => return Err(ConversionError::FieldTypeMissing(explicit_name)),
      }
    }
  }
}
