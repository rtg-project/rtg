use super::conversion_error::ConversionError;
use crate::explicit_model::ExplicitField;
use crate::implicit_model::ImplicitField;

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
    } => match name {
      Some(name) => match sql_type {
        Some(sql_type) => Ok(ExplicitField::ScalarDatabaseColumn {
          name: name.to_string(),
          nullable: nullable.unwrap_or(true),
          sql_type: (*sql_type).clone(),
          sql_column_name: sql_column_name.as_ref().unwrap_or(name).to_string(),
          graphql_field_name: graphql_field_name.as_ref().unwrap_or(name).to_string(),
          graphql_type_name: graphql_type_name.as_ref().unwrap_or(name).to_string(),
          graphql_order_by_asc: graphql_order_by_asc.as_ref().unwrap_or(name).to_string(),
          graphql_order_by_desc: graphql_order_by_desc.as_ref().unwrap_or(name).to_string(),
        }),
        None => return Err(ConversionError::FieldNameMissing),
      },
      None => return Err(ConversionError::FieldNameMissing),
    },
  }
}
