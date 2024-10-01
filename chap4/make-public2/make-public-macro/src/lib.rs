extern crate core;

use quote::{quote, ToTokens};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, parse::Parse};

struct StructField {
    name: syn::Ident,
    ty: syn::Type,
}

struct StructField2 {
    name: syn::Ident,
    ty: syn::Ident,
}

impl StructField {
    fn new(field: &syn::Field) -> Self {
        Self {
            name: field.ident.as_ref().unwrap().clone(),
            ty: field.ty.clone(),
        }
    }
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote! (pub #n: #t).to_tokens(tokens)
    }
}

struct UnnamedStructField(syn::Type);

impl UnnamedStructField {
    fn new(field: &syn::Field) -> Self {
        Self(field.ty.clone())
    }
}

impl ToTokens for UnnamedStructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let t = &self.0;
        quote!(pub #t).to_tokens(tokens)
    }
}

impl Parse for StructField2 {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // discarding first part of the parse-stream since it is the visibility
        let _vis: Result<syn::Visibility, _> = input.parse();
        let list =
            // A punctuated sequence of syntax tree nodes of type `T` separated by punctuation of type `P`.
            // here `P` is Colon or `:` because we are working with struct fields {... name: Type, ...}
            syn::punctuated::Punctuated::<syn::Ident, syn::token::Colon>
                ::parse_terminated(input).unwrap();
        Ok(StructField2 {
            name: list.first().unwrap().clone(), // first element should be the name
            ty: list.last().unwrap().clone(), // .. and last should be the type
        })
    }
}

impl ToTokens for StructField2 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote! (pub #n: #t).to_tokens(tokens)
    }
}

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    // print debugging, to help us with the development of the macro
    // using error-print to ensure output is not swallowed anywhere
    eprintln!("{:#?}", &ast);

    match ast.data {
        syn::Data::Struct (
            syn::DataStruct {fields: syn::Fields::Named(syn::FieldsNamed {ref named, ..}), ..}
        ) => {
            let name = ast.ident;
            let builder_fields = named.iter().map(StructField::new);
            let attrs = ast.attrs.iter();
            let public_version = quote! {
                #(#attrs)*
                pub struct #name {
                    #(#builder_fields,)*
                }
            };
            public_version.into()
        },
        syn::Data::Struct (
            syn::DataStruct {fields: syn::Fields::Unnamed(syn::FieldsUnnamed {ref unnamed, ..}), ..}
        ) => {
            let name = ast.ident;
            let builder_fields =
                unnamed.iter().map(UnnamedStructField::new);
            let attrs = ast.attrs.iter();
            let public_version = quote! {
                #(#attrs)*
                pub struct #name (
                    #(#builder_fields,)*
                );
            };
            public_version.into()
        },
        syn::Data::Enum(
            syn::DataEnum {ref variants, ..}
        ) => {
            let name = ast.ident;
            let builder_fields = variants.iter();
            let attrs = ast.attrs.iter();
            let public_version = quote! {
                #(#attrs)*
                pub enum #name {
                    #(#builder_fields,)*
                }
            };
            public_version.into()
        },
        //) => ast.to_token_stream().into(), // return the tokenstream as-is with no modifications
        _ => unimplemented!("only works for structs with named fields"),
    }
}

    //let builder_fields = fields.iter()
    //    .map(|f| {
    //        syn::parse2::<StructField2>(f.to_token_stream())
    //            .unwrap()
    //    });

// EXERCISES
//
// - exercise 1:
#[proc_macro_attribute]
pub fn delete(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let public_version = quote! {};
    public_version.into()
}
