use quote::{quote, ToTokens};
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated, Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed, Ident, Token
};

fn generated_methods(ast: &DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let named_fields = match ast.data
        { Struct
            ( DataStruct
              { fields: Named
                  ( FieldsNamed
                    { ref named
                    , ..
                    })
              , ..
              }
            ) => named
        , _ => unimplemented!("only workds for structs with named fields"),
        };
    named_fields.iter()
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            let type_name = &f.ty;
            let method_name =
                Ident::new(
                    &format!("get_{field_name}"),
                    Span::call_site(),
                );
            quote!(
                pub fn #method_name(&self) -> &#type_name {
                    &self.#field_name
                }
            )
    }).collect()
}

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    let item_as_stream: quote::__private::TokenStream =
        item.clone().into();
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let methods = generated_methods(&ast);

    quote!(
        #item_as_stream

        impl #name {
            #(#methods)*
        }
    ).into()
}

// Demonstrating Span or scope of values

#[proc_macro]
pub fn local_broken(_: TokenStream) -> TokenStream {
    // mixed_site will not be visible at the call site and trying to access `greeting` will not
    // compile.
    // Note that when using `cargo expand` it will look as though `greeting` should be in scope and
    // accessible, but in reality it is not.
    let greeting = Ident::new("greeting", Span::mixed_site());
    quote!(
        let #greeting = "Heya! It's me, Imoen!";
    ).into()
}

#[proc_macro]
pub fn local_fixed(_: TokenStream) -> TokenStream {
    // call_site will be visible at the call site and the value can be accessed.
    let greeting = Ident::new("greeting", Span::call_site());
    quote!(
        let #greeting = "Heya! It's me, Imoen!";
    ).into()
}
// Note `quote!` seems to do the same `call_site` when we are not creating a custom identifier
// through `Ident`
#[proc_macro]
pub fn local_fixed2(_item: TokenStream) -> TokenStream {
    quote!(
        let greeting = "Heya! It's me, Imoen!";
    ).into()
}

// chapter 5.3 - Compose macro

struct ComposeInput {
    expressions: Punctuated::<Ident, Token!(>>)>
}

impl Parse for ComposeInput {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(ComposeInput {expressions: Punctuated::<Ident, Token!(>>)>::parse_terminated(input).unwrap()})
    }
}

impl ToTokens for ComposeInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let xs: Vec<&Ident> = self.expressions.iter().collect();
        let result: Option<proc_macro2::TokenStream> = xs
            .split_last()
            .and_then(|(&last,xs)| {
                xs
                    .iter()
                    .rev()
                    .fold(None, |acc,expr| {
                        Some(match acc {
                            Some(prev) => quote!(compose_two_fwrd(#expr, #prev)),
                            None => quote!(compose_two_fwrd(#expr, #last))
                        })
                    })
            });
        result.to_tokens(tokens);
    }
}

#[proc_macro]
pub fn compose(item: TokenStream) -> TokenStream {
    let ci: ComposeInput = parse_macro_input!(item);

    quote!({
        fn compose_two_fwrd<FIRST, SECOND, THIRD, F, G>(first: F, second: G) -> impl Fn(FIRST) -> THIRD
        where
            F: Fn(FIRST) -> SECOND,
            G: Fn(SECOND) -> THIRD,
        {
            move |x| second(first(x))
        }
        #ci
    }).into()
}

// exercise 1 - use function style macro to add hello method to struct
#[proc_macro]
pub fn add_hello(item: TokenStream) -> TokenStream {
    // clone and convert the input token-stream so we can return it unaffected
    let item_as_token_stream: proc_macro2::TokenStream = item.clone().into();
    let ast = parse_macro_input!(item as DeriveInput);
    //name of the struct
    // TODO do we have to do any more verification to check that its a struct?
    let name = &ast.ident;
    quote!(
        #item_as_token_stream

        impl #name {
            pub fn hello(&self) {
                println!("hello world");
            }
        }
    ).into()
}

#[proc_macro]
pub fn add_hello2(struct_name: TokenStream) -> TokenStream {
    let struct_name = parse_macro_input!(struct_name as Ident);
    quote!(
        impl #struct_name {
            pub fn hello_world(&self) {
                println!("hello world");
            }
        }
    ).into()
}

// exercise 2 - create a version of the private macro where all fields are private and getters are
// public
#[proc_macro]
pub fn private2(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    match ast.data {
        syn::Data::Struct(
            syn::DataStruct {fields: syn::Fields::Named(syn::FieldsNamed {ref named, ..}), ..}
        ) => {
            let struct_name = ast.ident;
            let (struct_field_names_and_types,getter_fns): (Vec<_>,Vec<_>) =
                named.iter()
                .map(|f| {
                    let n = f.ident.as_ref().unwrap();
                    let t = &f.ty;
                    let new_struct_field = quote!(#n: #t);
                    let getter_name =
                        Ident::new(
                            &format!("get_{n}"),
                            Span::call_site()
                        );
                    let getter =
                        quote!(
                            pub fn #getter_name(&self) -> &#t {
                                &self.#n
                            }
                        );
                    ((new_struct_field,n), getter)
                }).unzip();
            let (struct_fields,struct_field_names): (Vec<_>,Vec<_>) =
                struct_field_names_and_types.into_iter().unzip();
            quote!(
                pub struct #struct_name {
                    #(#struct_fields,)*
                }
                impl #struct_name {
                    pub fn new(#(#struct_fields,)*) -> Self {
                        Self {#(#struct_field_names,)*}
                    }
                    #(#getter_fns)*
                }
            ).into()
        },
        _ => unimplemented!("only works for structs with named fields"),
    }
}
