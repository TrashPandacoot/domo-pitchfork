use std::iter::FromIterator;
mod generate_dataset_schema;
use generate_dataset_schema::expand_dataset_schema;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Attribute, AttributeArgs, DeriveInput, Error, ItemFn, Meta, NestedMeta, parse_macro_input};
use quote::quote;
#[proc_macro_derive(Domo, attributes(domo))]
pub fn generate_dataset_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_dataset_schema(input).unwrap_or_else(|err| syn::Error::to_compile_error(&err)).into()
}

#[proc_macro_attribute]
pub fn sv_domo(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let domo_meta = args.into_iter().map(|f| {
        let meta = match f {
            syn::NestedMeta::Meta(inner) => {
                // println!("{}", inner.path());
                println!("{}", inner.clone().into_token_stream().to_string());
                let val = match inner {
                    syn::Meta::Path(_) => {
                        panic!("don't use a path")
                    }
                    syn::Meta::List(l) => {
                        // println!("{}", l)
                        panic!("don't use a MetaList")
                    }
                    syn::Meta::NameValue(named_val) => {
                        // println!("{}", named_val.to_string());
                        // println!("{}", named_val.path.get_ident());
                        // println!("{}", named_val.lit);
                        let l = match named_val.lit {
                            syn::Lit::Str(s) => { s.value() }
                            // syn::Lit::ByteStr(s) => { s.value(). }
                            _ => panic!("not supported Lit type")
                        };
                        l
                    }
                };
                val
            }
            syn::NestedMeta::Lit(_) => {
                panic!("Literals aren't supported. see documentation for examples of how to use this sv_domo attribute")
            }
        };
    });
    let input = parse_macro_input!(input as ItemFn);
    input.into_token_stream().into()
}

fn get_domo_meta_items(attr: &Attribute) -> Result<Vec<NestedMeta>, Error> {
    if attr.path.is_ident("sv_domo") {
        match attr.parse_meta()? {
            Meta::List(meta) => Ok(Vec::from_iter(meta.nested)),
            bad => Err(Error::new_spanned(bad, "unrecognized attribute")),
        }
    } else {
        Ok(Vec::new())
    }
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}