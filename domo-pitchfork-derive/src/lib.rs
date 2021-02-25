mod generate_dataset_schema;
use generate_dataset_schema::expand_dataset_schema;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Domo, attributes(domo))]
pub fn generate_dataset_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_dataset_schema(input).into()
}