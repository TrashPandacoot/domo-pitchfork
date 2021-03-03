mod generate_dataset_schema;
use generate_dataset_schema::expand_dataset_schema;
use proc_macro::TokenStream;

use syn::{DeriveInput, parse_macro_input};
use quote::quote;

#[proc_macro_derive(Domo, attributes(domo))]
pub fn generate_dataset_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_dataset_schema(input).unwrap_or_else(to_compile_errors).into()
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
