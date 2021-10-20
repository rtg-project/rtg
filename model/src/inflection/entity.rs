use inflector::Inflector;

pub fn inflect_name_from_sql_table_name(name: &str) -> String {
  Inflector::to_pascal_case(&Inflector::to_singular(name))
}

pub fn inflect_name_from_graphql_entity_type_name(name: &str) -> String {
  Inflector::to_pascal_case(&Inflector::to_singular(name))
}

pub fn inflect_sql_table_name_from_name(name: &str) -> String {
  Inflector::to_snake_case(&Inflector::to_plural(name))
}

pub fn inflect_graphql_entity_type_name_from_name(name: &str) -> String {
  Inflector::to_pascal_case(&Inflector::to_singular(name))
}

pub fn inflect_graphql_filter_type_name_from_name(name: &str) -> String {
  format!(
    "{}WhereInput",
    Inflector::to_pascal_case(&Inflector::to_singular(name))
  )
}

pub fn inflect_graphql_get_single_operation_name_from_name(name: &str) -> String {
  format!(
    "get{}",
    Inflector::to_pascal_case(&Inflector::to_singular(name))
  )
}

pub fn inflect_graphql_get_list_operation_name_from_name(name: &str) -> String {
  format!(
    "all{}",
    Inflector::to_pascal_case(&Inflector::to_plural(name))
  )
}

pub fn inflect_graphql_get_connection_operation_name_from_name(name: &str) -> String {
  format!(
    "all{}Connection",
    Inflector::to_pascal_case(&Inflector::to_plural(name))
  )
}
