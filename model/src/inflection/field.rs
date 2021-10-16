use inflector::Inflector;

pub fn inflect_name_from_sql_column_name(name: &str) -> String {
  Inflector::to_camel_case(name)
}

pub fn inflect_sql_column_name_from_name(name: &str) -> String {
  Inflector::to_snake_case(name)
}

pub fn inflect_graphql_field_name_from_name(name: &str) -> String {
  Inflector::to_camel_case(name)
}

pub fn inflect_graphql_order_by_asc_from_name(name: &str) -> String {
  format!("{}Asc", Inflector::to_camel_case(name))
}

pub fn inflect_graphql_order_by_desc_from_name(name: &str) -> String {
  format!("{}Desc", Inflector::to_camel_case(name))
}
