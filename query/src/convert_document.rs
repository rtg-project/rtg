use super::conversion_error::ConversionError;
use super::convert_query::convert_query;
use graphql_parser::query::{Definition, Document, OperationDefinition, Text};
use rtg_model::model_cache::model_cache::ModelCache;

pub fn convert_document<'a, T: Text<'a>>(
  document: &Document<'a, T>,
  context: &ModelCache,
) -> Result<String, ConversionError> {
  if document.definitions.len() != 1 {
    return Err(ConversionError::DefinitionNumber);
  }

  match document.definitions.first() {
    Some(def) => match *def {
      Definition::Operation(ref op) => match *op {
        OperationDefinition::SelectionSet(ref _set) => {
          return Err(ConversionError::UnsupportedSyntax(
            "SelectionSet".to_string(),
          ))
        }
        OperationDefinition::Query(ref q) => return convert_query(q, context, "__rtg_0"),
        OperationDefinition::Mutation(ref _mut) => {
          return Err(ConversionError::UnsupportedSyntax("Mutation".to_string()))
        }
        OperationDefinition::Subscription(ref _sub) => {
          return Err(ConversionError::UnsupportedSyntax(
            "Subscription".to_string(),
          ))
        }
      },
      Definition::Fragment(ref _frag) => {
        return Err(ConversionError::UnsupportedSyntax("Fragment".to_string()))
      }
    },
    None => return Err(ConversionError::DefinitionNumber),
  };
}

// Tests
#[cfg(test)]
mod tests {
  use similar_asserts::assert_eq;
  #[test]
  fn it_works() {
    assert_eq!(1, 1);
  }
}
