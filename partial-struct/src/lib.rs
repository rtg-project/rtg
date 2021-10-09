extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;
use std::collections::HashMap;
use syn::{
  parse_derive_input, Body, DeriveInput, Field, Generics, Ident, Lit, MetaItem, NestedMetaItem,
  Variant, VariantData,
};

#[proc_macro_derive(
  PartialStruct,
  attributes(
    partial_name,
    partial_derive,
    partial_nested_original,
    partial_nested_generated
  )
)]
pub fn partial_struct(input: TokenStream) -> TokenStream {
  let s = input.to_string();
  let ast = parse_derive_input(&s).unwrap();
  let gen = generate_partial_struct(&ast);
  gen.parse().unwrap()
}

fn generate_partial_struct(ast: &DeriveInput) -> Tokens {
  match &ast.body {
    Body::Struct(ref variant_data) => match variant_data {
      VariantData::Struct(ref fields) => {
        let data = parse_struct_attributes(&ast);
        return create_struct(fields, data, &ast.generics);
      }
      VariantData::Tuple(_) => {
        panic!("PartialStruct does not support tuple variant in structs");
      }
      VariantData::Unit => {
        panic!("PartialStruct does not support unit variant in structs");
      }
    },
    Body::Enum(variants) => {
      let data = parse_enum_attributes(&ast);
      return create_enum(variants, data, &ast.generics);
    }
  }
}

struct StructAttributesData {
  original_struct_name: Ident,
  partial_struct_name: Ident,
  derives: Tokens,
  nested_names: HashMap<String, String>,
}

impl StructAttributesData {
  fn explode(self) -> (Ident, Ident, Tokens, HashMap<String, String>) {
    (
      self.original_struct_name,
      self.partial_struct_name,
      self.derives,
      self.nested_names,
    )
  }
}

struct EnumAttributesData {
  original_struct_name: Ident,
  partial_struct_name: Ident,
  derives: Tokens,
  nested_names: HashMap<String, String>,
}

fn nested_meta_item_to_ident(nested_item: &NestedMetaItem) -> &Ident {
  match nested_item {
    &NestedMetaItem::MetaItem(ref item) => match item {
      &MetaItem::Word(ref ident) => ident,
      _ => panic!("Only traits name are supported inside partial_struct"),
    },
    &NestedMetaItem::Literal(_) => {
      panic!("Only traits name are supported inside partial_struct")
    }
  }
}

fn create_nested_names_map(orig: Vec<Ident>, gen: Vec<Ident>) -> HashMap<String, String> {
  let mut map = HashMap::new();

  let orig_gen = orig.iter().zip(gen);

  for (orig, gen) in orig_gen {
    if gen.to_string().is_empty() {
      map.insert(orig.to_string(), "Partial".to_owned() + &gen.to_string());
    } else {
      map.insert(orig.to_string(), gen.to_string());
    }
  }

  map
}

fn handle_list(
  name: &Ident,
  values: &Vec<NestedMetaItem>,
  nested_original: &mut Vec<Ident>,
  nested_generated: &mut Vec<Ident>,
  derives: &mut Tokens,
) {
  match name.to_string().as_str() {
    "partial_derive" => {
      let mut derives_local = quote! {};
      for value in values {
        let derive_ident = nested_meta_item_to_ident(value);
        derives_local = quote! { #derive_ident, #derives_local }
      }
      *derives = derives_local;
    }
    "partial_nested_generated" => {
      for value in values {
        let generated_nested_name = nested_meta_item_to_ident(value);
        nested_generated.push(generated_nested_name.clone());
      }
    }
    "partial_nested_original" => {
      for value in values {
        let original_nested_name = nested_meta_item_to_ident(value);
        nested_original.push(original_nested_name.clone());
      }
    }
    _ => panic!("Only partial_derive are supported"),
  };
}

fn handle_name_value(name: &Ident, value: &Lit, struct_name: &mut Ident) {
  match value {
    &Lit::Str(ref name_value, _) => {
      if name == "partial_name" {
        *struct_name = Ident::new(name_value.clone())
      } else {
        panic!("Only partial_name is supported");
      }
    }
    _ => panic!("partial_name should be a string"),
  }
}

fn parse_struct_attributes(ast: &DeriveInput) -> StructAttributesData {
  let original_struct_name = ast.ident.clone();
  let mut struct_name = String::from("Partial");
  struct_name.push_str(&ast.ident.to_string());
  let mut struct_name = Ident::new(struct_name);
  let mut derives = quote! {};
  let mut nested_generated = Vec::new();
  let mut nested_original = Vec::new();

  for attribute in &ast.attrs {
    match &attribute.value {
      &MetaItem::Word(_) => panic!("No word attribute is supported"),
      &MetaItem::NameValue(ref name, ref value) => handle_name_value(name, value, &mut struct_name),
      &MetaItem::List(ref name, ref values) => handle_list(
        name,
        values,
        &mut nested_original,
        &mut nested_generated,
        &mut derives,
      ),
    }
  }

  // prevent warnings if no derive is given
  derives = if derives.to_string().is_empty() {
    quote! {}
  } else {
    quote! { #[derive(#derives)] }
  };

  StructAttributesData {
    original_struct_name: original_struct_name,
    partial_struct_name: struct_name,
    derives: derives,
    nested_names: create_nested_names_map(nested_original, nested_generated),
  }
}

fn parse_enum_attributes(ast: &DeriveInput) -> EnumAttributesData {
  let original_struct_name = ast.ident.clone();
  let mut struct_name = String::from("Partial");
  struct_name.push_str(&ast.ident.to_string());
  let mut struct_name = Ident::new(struct_name);
  let mut derives = quote! {};
  let mut nested_generated = Vec::new();
  let mut nested_original = Vec::new();

  for attribute in &ast.attrs {
    match &attribute.value {
      &MetaItem::Word(_) => panic!("No word attribute is supported"),
      &MetaItem::NameValue(ref name, ref value) => handle_name_value(name, value, &mut struct_name),
      &MetaItem::List(ref name, ref values) => handle_list(
        name,
        values,
        &mut nested_original,
        &mut nested_generated,
        &mut derives,
      ),
    }
  }

  // prevent warnings if no derive is given
  derives = if derives.to_string().is_empty() {
    quote! {}
  } else {
    quote! { #[derive(#derives)] }
  };

  EnumAttributesData {
    original_struct_name: original_struct_name,
    partial_struct_name: struct_name,
    derives: derives,
    nested_names: create_nested_names_map(nested_original, nested_generated),
  }
}

/// Generate data for fields of a struct
fn create_fields(
  fields: &Vec<Field>,
  nested_names: HashMap<String, String>,
) -> (Tokens, Tokens, Tokens) {
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
    } else if nested_names.contains_key(&type_name_string) {
      let type_name = Ident::new(nested_names.get(&type_name_string).unwrap().as_str());
      next_attribute = quote! { pub #field_name: #type_name, };
      next_assigner = quote! { self.#field_name.apply_partials(partial_struct.#field_name); };
      next_empty = quote! { #field_name: #type_name::empty(), };
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
fn create_variant_struct(fields: &Vec<Field>, data: StructAttributesData, ident: Ident) -> Tokens {
  let (original_struct_name, partial_struct_name, derives, nested_names) = data.explode();
  let (assigners, attributes, empty) = create_fields(&fields, nested_names);

  quote! {
    #ident {
      #attributes
    }
  }
}

/// Generate data for the variants of an enum
fn create_variants(
  variants: &Vec<Variant>,
  data: StructAttributesData,
  nested_names: HashMap<String, String>,
) -> (Tokens, Tokens, Tokens) {
  let mut attributes = quote! {};
  let mut assigners = quote! {};
  let mut empty = quote! {};

  // for variant in variants {
  //   match variant.data {
  //     VariantData::Struct(ref fields) => {
  //       let a = create_variant_struct(fields, data, variant.ident);
  //       attributes = quote! {
  //         #attributes
  //       }
  //     }
  //     VariantData::Tuple(_) => {
  //       panic!("PartialStruct does not support tuple variant in structs");
  //     }
  //     VariantData::Unit => {
  //       panic!("PartialStruct does not support unit variant in structs");
  //     }
  //   }
  // }

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

/// Generates code for enum
fn create_enum(variants: &Vec<Variant>, data: EnumAttributesData, generics: &Generics) -> Tokens {
  // let (original_struct_name, partial_struct_name, derives, nested_names) = data.explode();
  // let (assigners, attributes, empty) = create_variants(&variants, data, nested_names);
  // let (_, generics_no_where, _) = generics.split_for_impl();

  // TODO: de-hardcode everything bellow
  quote! {
    // #derives
    // pub enum #partial_struct_name #generics {
    //   #attributes
    // }
  }
}

/// Generates code for simple structs
fn create_struct(fields: &Vec<Field>, data: StructAttributesData, generics: &Generics) -> Tokens {
  let (original_struct_name, partial_struct_name, derives, nested_names) = data.explode();
  let (assigners, attributes, empty) = create_fields(&fields, nested_names);

  let (_, generics_no_where, _) = generics.split_for_impl();

  quote! {
    #derives
    pub struct #partial_struct_name #generics {
      #attributes
    }

    impl #generics #original_struct_name #generics_no_where {
      pub fn apply_partials(&mut self, partial_struct: #partial_struct_name #generics_no_where) {
        #assigners
      }
    }

    impl #generics #partial_struct_name #generics_no_where {
      pub fn empty() -> #partial_struct_name #generics_no_where {
        #partial_struct_name {
          #empty
        }
      }
    }
  }
}
