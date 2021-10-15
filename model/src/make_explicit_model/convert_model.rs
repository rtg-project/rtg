use super::conversion_error::ConversionError;
use super::convert_entity::convert_entity;
use crate::explicit_model::{ExplicitEntity, ExplicitModel};
use crate::implicit_model::ImplicitModel;

use std::rc::Rc;

pub fn convert_model(implicit_model: &ImplicitModel) -> Result<ExplicitModel, ConversionError> {
  match &*implicit_model {
    ImplicitModel::V1 { entities } => Ok(ExplicitModel::V1 {
      entities: match entities {
        Some(entities) => entities
          .iter()
          .map(|entity| match convert_entity(entity) {
            Ok(entity) => Ok(Rc::new(entity)),
            Err(err) => Err(err),
          })
          .collect::<Result<Vec<Rc<ExplicitEntity>>, ConversionError>>()?,
        None => vec![],
      },
    }),
  }
}
