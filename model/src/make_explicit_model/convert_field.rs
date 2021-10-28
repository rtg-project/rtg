use super::conversion_error::ConversionError;
use crate::explicit_model::ExplicitField;
use crate::implicit_model::ImplicitField;
use crate::inflection;

pub fn convert_field(implicit_field: &ImplicitField) -> Result<ExplicitField, ConversionError> {
  match &*implicit_field {
    ImplicitField::ScalarDatabaseColumn {
      name,
      nullable,
      sql_type,
      sql_column_name,
      graphql_enabled,
      graphql_field_name,
      graphql_type_name,
      graphql_order_by_asc,
      graphql_order_by_desc,
    } => {
      // Infer the explicit name of the field
      let explicit_name = match name {
        Some(implicit_name) => implicit_name.to_owned(),
        None => match graphql_field_name {
          Some(implicit_graphql_field_name) => {
            inflection::field::inflect_name_from_graphql_field_name(implicit_graphql_field_name)
          }
          None => match sql_column_name {
            Some(implicit_sql_column_name) => {
              inflection::field::inflect_name_from_sql_column_name(implicit_sql_column_name)
            }
            None => return Err(ConversionError::FieldNameMissing),
          },
        },
      };
      // Infer the type of the field
      let (explicit_sql_type, explicit_graphql_type_name, _is_array) = match sql_type {
        Some(sql_type) => {
          let graphql_inflection_result =
            inflection::sql_type::inflect_graphql_type_from_sql_type(sql_type);
          let graphql_inflection = match graphql_inflection_result {
            Ok(graphql_inflection) => graphql_inflection,
            Err(error) => {
              return Err(ConversionError::FieldInflection(
                explicit_name,
                format!("{}", error),
              ))
            }
          };
          match graphql_type_name {
            Some(graphql_type_name) => {
              if graphql_inflection.0 == *graphql_type_name {
                (
                  sql_type.to_owned(),
                  graphql_inflection.0,
                  graphql_inflection.1,
                )
              } else {
                return Err(ConversionError::FieldTypeMismatch(
                  explicit_name,
                  graphql_type_name.to_owned(),
                  format!("{:?}", sql_type),
                ));
              }
            }
            None => (
              sql_type.to_owned(),
              graphql_inflection.0,
              graphql_inflection.1,
            ),
          }
        }
        None => match graphql_type_name {
          Some(graphql_type_name) => {
            let sql_inflection_result =
              inflection::sql_type::inflect_sql_type_from_graphql_type(graphql_type_name, false);
            let sql_inflection = match sql_inflection_result {
              Ok(sql_inflection) => sql_inflection,
              Err(error) => {
                return Err(ConversionError::FieldInflection(
                  explicit_name,
                  format!("{}", error),
                ))
              }
            };
            (sql_inflection, graphql_type_name.to_owned(), false)
          }
          None => return Err(ConversionError::FieldTypeMissing(explicit_name)),
        },
      };
      Ok(ExplicitField::ScalarDatabaseColumn {
        name: explicit_name.to_owned(),
        nullable: nullable.unwrap_or(true),
        sql_type: explicit_sql_type,
        sql_column_name: sql_column_name
          .as_ref()
          .unwrap_or(&inflection::field::inflect_sql_column_name_from_name(
            &explicit_name,
          ))
          .to_owned(),
        graphql_enabled: graphql_enabled.unwrap_or(true),
        graphql_field_name: graphql_field_name
          .as_ref()
          .unwrap_or(&inflection::field::inflect_graphql_field_name_from_name(
            &explicit_name,
          ))
          .to_owned(),
        graphql_type_name: explicit_graphql_type_name,
        graphql_order_by_asc: graphql_order_by_asc
          .as_ref()
          .unwrap_or(&inflection::field::inflect_graphql_order_by_asc_from_name(
            &explicit_name,
          ))
          .to_owned(),
        graphql_order_by_desc: graphql_order_by_desc
          .as_ref()
          .unwrap_or(&inflection::field::inflect_graphql_order_by_desc_from_name(
            &explicit_name,
          ))
          .to_owned(),
      })
    }
  }
}
