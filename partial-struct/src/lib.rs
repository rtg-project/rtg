extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;
use std::collections::HashMap;
use syn::{
  parse_derive_input, Attribute, Body, DeriveInput, Field, Generics, Ident, Lit, MetaItem,
  NestedMetaItem, Variant, VariantData,
};

/// The entry point of this macro
#[proc_macro_derive(PartialStruct, attributes(partial, partial_attribute))]
pub fn partial_struct(input: TokenStream) -> TokenStream {
  let s = input.to_string();
  let ast = parse_derive_input(&s).unwrap();
  let gen = match &ast.body {
    Body::Enum(variants) => create_enum(&ast, variants),
    Body::Struct(ref variant_data) => match variant_data {
      VariantData::Struct(ref fields) => {
        // Normal struct
        create_struct(&ast, fields)
      }
      VariantData::Tuple(_) => {
        panic!("PartialStruct does not support tuple variant in structs");
      }
      VariantData::Unit => {
        panic!("PartialStruct does not support unit variant in structs");
      }
    },
  };
  return gen.parse().unwrap();
}

/// A struct that contains the information about the attributes
/// of an object  (enum, struct, variant or field)
struct AttributeData {
  name: Ident,
  completion: Option<String>,
  require: bool,
  skip: bool,
  attributes: Vec<Tokens>,
}

/// Parses the `partial` and `partial_attribute` attributes
/// and returns an AttributeData struct that contains important info about the object.
/// This applies to struct, enums and variants
fn parse_attributes(name: &String, attrs: &Vec<Attribute>) -> AttributeData {
  let mut partial_name = name.clone();
  let mut partial_completion = None;
  let mut attributes = Vec::new();
  let mut skip = false;
  let mut require = false;
  for attribute in &*attrs {
    match &attribute.value {
      &MetaItem::Word(ref name) => match name.to_string().as_str() {
        "partial" => panic!("partial must be a list meta item"),
        "partial_attribute" => panic!("partial_attribute must be a list meta item"),
        _ => {} // Just skip unknown attributes
      },
      &MetaItem::NameValue(ref name, ref value) => match name.to_string().as_str() {
        "partial" => match value {
          _ => panic!("partial must be a list meta item"),
        },
        "partial_attribute" => match value {
          _ => panic!("partial_attribute must be a list meta item"),
        },
        _ => {} // Just skip unknown attributes
      },
      &MetaItem::List(ref name, ref values) => {
        match name.to_string().as_str() {
          "partial" => {
            for value in values {
              match value {
                NestedMetaItem::Literal(lit) => match lit {
                  &Lit::Str(ref name_value, _) => match name_value.as_str() {
                    "skip" => skip = true,
                    "require" => require = true,
                    _ => panic!("Unknown argument for partial: {}", name_value),
                  },
                  _ => panic!("partial name should be a string"),
                },
                NestedMetaItem::MetaItem(meta_item) => match meta_item {
                  MetaItem::Word(ref name2) => match name2.to_string().as_str() {
                    "skip" => skip = true,
                    "require" => require = true,
                    _ => panic!(
                      "item {:?} not recognized for the `partial` attribute.",
                      name2
                    ),
                  },
                  MetaItem::List(ref name2, ref nestedMetaItems) => {
                    panic!("List({:?})", meta_item)
                  }
                  MetaItem::NameValue(ref name2, ref lit2) => match name2.to_string().as_str() {
                    "name" => match lit2 {
                      &Lit::Str(ref name_value, _) => partial_name = format!("{}", name_value),
                      _ => panic!("partial name should be a string"),
                    },
                    "completion" => match lit2 {
                      &Lit::Str(ref name_value, _) => {
                        partial_completion = Some(format!("{}", name_value))
                      }
                      _ => panic!("partial completion should be a string, the name of a function"),
                    },
                    "skip" => match lit2 {
                      &Lit::Bool(ref value) => skip = *value,
                      _ => panic!("partial skip should be a boolean"),
                    },
                    "require" => match lit2 {
                      &Lit::Bool(ref value) => require = *value,
                      _ => panic!("partial require should be a boolean"),
                    },
                    _ => panic!(
                      "item {:?} not recognized for the `partial` attribute.",
                      lit2
                    ),
                  },
                },
              }
            }
          }
          "partial_attribute" => {
            for value in values {
              let mut tokens = quote::Tokens::default();
              quote::ToTokens::to_tokens(value, &mut tokens);
              attributes.push(tokens);
            }
          }
          _ => {} // Just skip unknown attributes
        };
      }
    }
  }
  AttributeData {
    name: Ident::new(partial_name),
    completion: partial_completion,
    require,
    skip,
    attributes,
  }
}

/// Generates code for enum
fn create_enum(ast: &DeriveInput, variants: &Vec<Variant>) -> Tokens {
  let AttributeData {
    attributes,
    completion,
    name,
    require,
    skip,
  } = parse_attributes(&format!("{}Partial", ast.ident), &ast.attrs);
  let (assigners, variants_result, empty) = create_variants(&variants);
  // let original_struct_name = ast.ident.clone();
  let generics = ast.generics.clone();
  // let (_impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

  // TODO: de-hardcode everything bellow
  quote! {
    #attributes
    pub enum #name #generics {
      #variants_result
    }
  }
}

fn create_struct(ast: &DeriveInput, fields: &Vec<Field>) -> Tokens {
  let AttributeData {
    attributes,
    completion,
    name,
    require,
    skip,
  } = parse_attributes(&format!("{}Partial", ast.ident), &ast.attrs);
  let original_struct_name = ast.ident.clone();
  let (assigners, fields, empty) = create_fields(&fields);
  let generics = ast.generics.clone();
  let (_impl_generics, ty_generics, _where_clause) = generics.split_for_impl();
  return quote! {
    #attributes
    pub struct #name #generics {
      #fields
    }
    impl #generics #original_struct_name #ty_generics {
      pub fn apply_partials(&mut self, partial_struct: #name #ty_generics) {
        #assigners
      }
    }
    impl #generics #name #ty_generics {
      pub fn empty() -> #name #ty_generics {
        #name {
          #empty
        }
      }
    }
  };
}

/// Generate data for fields of a struct
fn create_fields(fields: &Vec<Field>) -> (Tokens, Tokens, Tokens) {
  let mut attributes = quote! {};
  let mut assigners = quote! {};
  let mut empty = quote! {};
  for field in fields {
    let ref type_name = &field.ty;
    let ref field_name = &field.ident.clone().unwrap();
    let next_attribute;
    let next_assigner;
    let next_empty;

    let type_name_string = quote! {#type_name}.to_string();
    let type_name_string: String = type_name_string.chars().filter(|&c| c != ' ').collect();

    if type_name_string.starts_with("Option<") {
      next_attribute = quote! { pub #field_name: #type_name, };
      next_assigner = quote! { self.#field_name = partial_struct.#field_name; };
      next_empty = quote! { #field_name: None, };
    // } else if nested_names.contains_key(&type_name_string) {
    //   let type_name = Ident::new(nested_names.get(&type_name_string).unwrap().as_str());
    //   next_attribute = quote! { pub #field_name: #type_name, };
    //   next_assigner = quote! { self.#field_name.apply_partials(partial_struct.#field_name); };
    //   next_empty = quote! { #field_name: #type_name::empty(), };
    } else {
      next_attribute = quote! { pub #field_name: Option<#type_name>, };
      next_assigner = quote! {
          if let Some(attribute) = partial_struct.#field_name {
              self.#field_name = attribute;
          }
      };
      next_empty = quote! { #field_name: None, };
    }

    assigners = quote! { #assigners #next_assigner };
    attributes = quote! { #attributes #next_attribute };
    empty = quote! { #empty #next_empty }
  }

  (assigners, attributes, empty)
}

/// Generates code for simple structs
fn create_variant(fields: &Vec<Field>, variant_attributes_data: AttributeData) -> Tokens {
  let AttributeData {
    name,
    completion,
    skip,
    require,
    attributes,
  } = variant_attributes_data;
  let (assigners, attributes, empty) = create_fields(&fields);

  if skip {
    return quote! {};
  }

  quote! {
    #name {
      #attributes
    }
  }
}

/// Generate data for the variants of an enum
fn create_variants(variants: &Vec<Variant>) -> (Tokens, Tokens, Tokens) {
  let mut attributes = quote! {};
  let mut assigners = quote! {};
  let mut empty = quote! {};

  for variant in variants {
    match variant.data {
      VariantData::Struct(ref fields) => {
        // let variant_attributes_data = parse_variant_attributes(variant);
        // let variant_name = variant.ident.clone();
        let variant = create_variant(fields, variant_attributes_data, variant_name);
        attributes = quote! {
          #attributes
          // #variant
        }
      }
      VariantData::Tuple(_) => {
        panic!("PartialStruct does not support tuple variant in structs so far");
      }
      VariantData::Unit => {
        panic!("PartialStruct does not support unit variant in structs so far");
      }
    }
  }

  //////////////////////////
  // for field in fields {
  //   let ref type_name = &field.ty;
  //   let ref field_name = &field.ident.clone().unwrap();
  //   let next_attribute;
  //   let next_assigner;
  //   let next_empty;

  //   let type_name_string = quote! {#type_name}.to_string();
  //   let type_name_string: String = type_name_string.chars().filter(|&c| c != ' ').collect();

  //   if type_name_string.starts_with("Option<") {
  //     next_attribute = quote! { pub #field_name: #type_name, };
  //     next_assigner = quote! { self.#field_name = partial_struct.#field_name; };
  //     next_empty = quote! { #field_name: None, };
  //   } else if nested_names.contains_key(&type_name_string) {
  //     let type_name = Ident::new(nested_names.get(&type_name_string).unwrap().as_str());
  //     next_attribute = quote! { pub #field_name: #type_name, };
  //     next_assigner = quote! { self.#field_name.apply_partials(partial_struct.#field_name); };
  //     next_empty = quote! { #field_name: #type_name::empty(), };
  //   } else {
  //     next_attribute = quote! { pub #field_name: Option<#type_name>, };
  //     next_assigner = quote! {
  //         if let Some(attribute) = partial_struct.#field_name {
  //             self.#field_name = attribute;
  //         }
  //     };
  //     next_empty = quote! { #field_name: None, };
  //   }

  //   assigners = quote! { #assigners #next_assigner };
  //   attributes = quote! { #attributes #next_attribute };
  //   empty = quote! { #empty #next_empty }
  // }

  (assigners, attributes, empty)
}

// /// Generates code for simple structs
// fn create_struct(
//   fields: &Vec<Field>,
//   struct_attributes_data: AttributeData,
//   generics: &Generics,
// ) -> Tokens {
//   let AttributeData {
//     attributes,
//     name,
//     require,
//     skip,
//   } = struct_attributes_data;
//   let (assigners, attributes, empty) = create_fields(&fields);

//   let (_impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

//   quote! {
//     #attributes
//     pub struct #name #generics {
//       #attributes
//     }

//     impl #generics #original_struct_name #ty_generics {
//       pub fn apply_partials(&mut self, partial_struct: #name #ty_generics) {
//         #assigners
//       }
//     }

//     impl #generics #name #ty_generics {
//       pub fn empty() -> #name #ty_generics {
//         #name {
//           #empty
//         }
//       }
//     }
//   }
// }
