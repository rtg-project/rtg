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
  let ResultData {
    partial_declaration,
    merge_assigner,
    empty_initializer,
  } = match &ast.body {
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
  // panic!("{}", partial_declaration.parse::<String>().unwrap());
  let result = quote! {
    /// Partial declaration
    #[automatically_derived]
    #partial_declaration

    /// Function to merge with a complete
    #[automatically_derived]
    #merge_assigner

    /// Function to create an empty partial
    #[automatically_derived]
    #empty_initializer
  };
  return result.parse().unwrap();
}

/// A struct that contains the information about the attributes
/// of an object  (enum, struct, variant or field)
struct AttributeData {
  name: Ident,
  completion: Option<String>,
  require: bool,
  skip: bool,
  attributes: Tokens,
}

/// A struct that contains the result of "partialization" of
/// an object  (enum, struct, variant or field)
struct ResultData {
  partial_declaration: Tokens,
  empty_initializer: Tokens,
  merge_assigner: Tokens,
}

/// Parses the `partial` and `partial_attribute` attributes
/// and returns an AttributeData struct that contains important info about the object.
/// This applies to struct, enums and variants
fn parse_attributes(name: &String, attrs: &Vec<Attribute>) -> AttributeData {
  let mut partial_name = name.clone();
  let mut partial_completion = None;
  let mut attributes = quote! {};
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
                  MetaItem::List(ref name2, ref nested_meta_items) => {
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
              attributes = quote! {
                #attributes
                #[#tokens]
              };
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
fn create_enum(ast: &DeriveInput, variants: &Vec<Variant>) -> ResultData {
  let AttributeData {
    attributes,
    completion,
    name,
    require,
    skip,
  } = parse_attributes(&format!("{}Partial", ast.ident), &ast.attrs);

  let ResultData {
    partial_declaration,
    empty_initializer,
    merge_assigner,
  } = create_variants(&variants);
  let original_name = ast.ident.clone();
  let generics = ast.generics.clone();
  let (_impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

  ResultData {
    partial_declaration: quote! {
      #attributes
      pub enum #name #generics {
        #partial_declaration
      }
    },
    merge_assigner: quote! {
     // #merge_assigner
    },
    empty_initializer: quote! {
     // #empty_initializer
    },
  }
}

/// Generate data for the variants of an enum
fn create_variants(variants: &Vec<Variant>) -> ResultData {
  let mut partial_declaration_mut = quote! {};
  let mut merge_assigner_mut = quote! {};
  let mut empty_initializer_mut = quote! {};

  for variant in variants {
    let ResultData {
      partial_declaration,
      empty_initializer,
      merge_assigner,
    } = create_variant(variant);
    partial_declaration_mut = quote! {
      #partial_declaration_mut
      #partial_declaration
    }
  }

  ResultData {
    merge_assigner: quote! {
      #merge_assigner_mut
    },
    empty_initializer: quote! {
      #empty_initializer_mut
    },
    partial_declaration: quote! {
      #partial_declaration_mut
    },
  }
}

/// Generates code for simple structs
fn create_variant(variant: &Variant) -> ResultData {
  let mut partial_declaration_mut = quote! {};
  let mut merge_assigner_mut = quote! {};
  let mut empty_initializer_mut = quote! {};

  let AttributeData {
    name,
    completion,
    skip,
    require,
    attributes,
  } = parse_attributes(&format!("{}", variant.ident), &variant.attrs);

  let original_name = variant.ident.clone();

  match variant.data {
    VariantData::Struct(ref fields) => {
      let ResultData {
        partial_declaration,
        empty_initializer,
        merge_assigner,
      } = create_fields(&fields);
      partial_declaration_mut = quote! {
        #partial_declaration
      };
    }
    VariantData::Tuple(_) => {
      panic!("PartialStruct does not support tuple variant in structs so far");
    }
    VariantData::Unit => {
      panic!("PartialStruct does not support unit variant in structs so far");
    }
  }

  if skip {
    return ResultData {
      partial_declaration: quote! {},
      empty_initializer: quote! {
        #empty_initializer_mut
      },
      merge_assigner: quote! {
        #merge_assigner_mut
      },
    };
  }

  ResultData {
    partial_declaration: quote! {
      #attributes
      #name {
        #partial_declaration_mut
      },
    },
    empty_initializer: quote! {
      #empty_initializer_mut
    },
    merge_assigner: quote! {
      #merge_assigner_mut
    },
  }
}

fn create_struct(ast: &DeriveInput, fields: &Vec<Field>) -> ResultData {
  let AttributeData {
    attributes,
    completion,
    name,
    require,
    skip,
  } = parse_attributes(&format!("{}Partial", ast.ident), &ast.attrs);
  let original_struct_name = ast.ident.clone();
  let ResultData {
    partial_declaration,
    empty_initializer,
    merge_assigner,
  } = create_fields(&fields);
  let generics = ast.generics.clone();
  let (_impl_generics, ty_generics, _where_clause) = generics.split_for_impl();
  return ResultData {
    partial_declaration: quote! {
      #attributes
      pub struct #name #generics {
        #partial_declaration
      }
    },
    merge_assigner: quote! {
      impl #generics #original_struct_name #ty_generics {
        pub fn apply_partials(&mut self, partial_struct: #name #ty_generics) {
          #merge_assigner
        }
      }
    },
    empty_initializer: quote! {
      impl #generics #name #ty_generics {
        pub fn empty() -> #name #ty_generics {
          #name {
            #empty_initializer
          }
        }
      }
    },
  };
}

/// Generate data for fields of a struct
fn create_fields(fields: &Vec<Field>) -> ResultData {
  let mut partial_declaration_mut = quote! {};
  let mut merge_assigner_mut = quote! {};
  let mut empty_initializer_mut = quote! {};
  for field in fields {
    let name_init = if field.ident.is_some() {
      format!("{}", field.ident.clone().unwrap())
    } else {
      format!("")
    };
    let AttributeData {
      attributes,
      completion,
      name,
      require,
      skip,
    } = parse_attributes(&name_init, &field.attrs);

    let ref type_name = &field.ty;
    let ref field_name = &field.ident.clone().unwrap();
    let next_partial_declaration;
    let next_merge_assigner;
    let next_empty_initializer;

    let type_name_string = quote! {#type_name}.to_string();
    let type_name_string: String = type_name_string.chars().filter(|&c| c != ' ').collect();

    if type_name_string.starts_with("Option<") {
      next_partial_declaration = quote! { #field_name: #type_name, };
      // next_partial_declaration = quote! { pub #field_name: #type_name, };
      next_merge_assigner = quote! { self.#field_name = partial_struct.#field_name; };
      next_empty_initializer = quote! { #field_name: None, };
    // } else if nested_names.contains_key(&type_name_string) {
    //   let type_name = Ident::new(nested_names.get(&type_name_string).unwrap().as_str());
    //   next_partial_declaration = quote! { pub #field_name: #type_name, };
    //   next_merge_assigner = quote! { self.#field_name.apply_partials(partial_struct.#field_name); };
    //   next_empty_initializer = quote! { #field_name: #type_name::empty(), };
    } else {
      next_partial_declaration = quote! { #field_name: Option<#type_name>, };
      // next_partial_declaration = quote! { pub #field_name: Option<#type_name>, };
      next_merge_assigner = quote! {
          if let Some(field_value) = partial_struct.#field_name {
              self.#field_name = field_value;
          }
      };
      next_empty_initializer = quote! { #field_name: None, };
    }

    merge_assigner_mut = quote! { #merge_assigner_mut #next_merge_assigner };
    partial_declaration_mut = quote! { #partial_declaration_mut #next_partial_declaration };
    empty_initializer_mut = quote! { #empty_initializer_mut #next_empty_initializer }
  }

  ResultData {
    partial_declaration: quote! {
      #partial_declaration_mut
    },
    empty_initializer: quote! {
      #empty_initializer_mut
    },
    merge_assigner: quote! {
      #merge_assigner_mut
    },
  }
}
