use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let name_str = name.to_string();
    let uppercase_name = name_str.to_uppercase();

    let add_impls = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("helloo world")
            }
            fn uppercase(&self) {
                println!("{}", #uppercase_name)
            }
            fn testing_testing() {
                println!("one two three")
            }
            fn greet(&self, input_name: &str) {
                println!("Hi {}, from {}", input_name, #name_str)
            }
        }
    };
    add_impls.into()
}
