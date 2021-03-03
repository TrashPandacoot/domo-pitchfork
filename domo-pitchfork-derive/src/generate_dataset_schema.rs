use std::{any::{Any}, collections::HashMap};

use proc_macro2::{Span, TokenStream};
use syn::{Data, DataStruct, DeriveInput, Field, Fields, GenericArgument, Lit, Meta, NestedMeta, Path, PathArguments, Type, spanned::Spanned};
use quote::{ToTokens, quote};

/// derive the DomoSchema trait from the domo_pitchfork crate.
pub fn expand_dataset_schema(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let st_name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = if let Data::Struct(DataStruct { fields: Fields::Named(fields), ..}) = input.data {
        fields.named
    } else {
        return Err(syn::Error::new(st_name.span(),"this derive macro only works on structs with named fields"))
    };

    let columns: Result<Vec<TokenStream>, syn::Error> = fields.into_iter().map(|f| {
        let r = {
            let name = get_domo_field_name(&f);
            let column_type = get_domo_column_type(&f)?;
            Ok(quote! {
                Column {
                    name: #name.to_owned(),
                    column_type: #column_type.to_owned(),
                }
            })
        };
        r
    }).collect();
    let columns = columns?;

    let tokens = quote! {
        #[automatically_derived]
        impl domo_pitchfork::domo::dataset::DomoSchema #impl_generics for #st_name #ty_generics #where_clause {
            fn domo_dataset_schema() -> domo_pitchfork::domo::dataset::Schema {
                let columns = vec![
                    #(#columns),*
                ];
                domo_pitchfork::domo::dataset::Schema {
                    columns,
                }
            }
        }
    };
    Ok(tokens)
}

/// Check to see if `domo` attribute proc_macro is being used to specify a Domo Column name for the field,
/// otherwise stringify the Rust field's name to use as a Domo Column name.
fn get_domo_field_name(f: &Field) -> String {
    let at = &f.attrs;
    let domo_attr = at.iter().find(|a| a.path.is_ident("domo"));
    match domo_attr {
        Some(d_attr) => {
            let dv = d_attr.parse_meta().unwrap();
            let v = match dv {
                syn::Meta::List(m) => { 
                    let list: Vec<String> = m.nested.iter().map(|nm| {
                        let out = match nm {
                            NestedMeta::Lit(_) => {
                                domo_column_name_from_ident(f)
                            }
                            NestedMeta::Meta(m) => {
                                let map = get_string_from_meta(m);
                                // dbg!(&map);
                                map.get("name").unwrap_or(&"".to_string()).to_string()
                            }
                        };
                        out
                    }).filter(|s| s != &"".to_string()).collect();
                    // dbg!(&list);
                    
                    let n = list.first().map_or_else(|| domo_column_name_from_ident(f) ,|v| v.to_string());
                    n
                },
                syn::Meta::Path(m) => { format!("{:?}", m.get_ident())},
                syn::Meta::NameValue(_) => { 
                    domo_column_name_from_ident(f)
                },
            };
            v
        }
        None => {
            domo_column_name_from_ident(f)
        }
    }
}

/// Stringify the Rust Field's name to use as a Domo Column name if the `domo` attribute wasn't
/// used to specify a Domo column name.
fn domo_column_name_from_ident(f: &Field) -> String {
    let n = &f.ident;
    let str_name_val = n.as_ref().unwrap().to_string();
    str_name_val
}

/// Infer Domo Column Type for some common Rust Types. Defaults to "STRING" if the type isn't
/// in the hardcoded list being checked. A user can manually specify a column type using the `domo`
/// attribute making hardcoding common types (such as numeric types) an acceptable solution here.
fn map_type_to_domo_type(s: String) -> String {
    match s.as_str() {
        "isize" => "LONG".to_string(),
        "usize" => "LONG".to_string(),
        "i8" => "LONG".to_string(),
        "i16" => "LONG".to_string(),
        "i32" => "LONG".to_string(),
        "i64" => "LONG".to_string(),
        "i128" => "LONG".to_string(),
        "u8" => "LONG".to_string(),
        "u16" => "LONG".to_string(),
        "u32" => "LONG".to_string(),
        "u64" => "LONG".to_string(),
        "u128" => "LONG".to_string(),
        "f32" => "DOUBLE".to_string(),
        "f64" => "DOUBLE".to_string(),
        _ => "STRING".to_string(),
    }
}

fn get_string_from_meta(m: &Meta) -> HashMap<String, String> {
    let mut map = HashMap::new();
    match m {
        syn::Meta::Path(m) => { 
            map.insert("get_string_from_meta Path".to_string(), format!("get_string_from_meta {:?}", m.type_id()));
        },
        syn::Meta::List(m) => { 
            m.nested.iter().for_each(|nm| {
                match nm {
                    NestedMeta::Lit(l) => {
                        match l {
                            Lit::Str(s) => map.insert("lit".to_string(), s.value()),
                            _ => map.insert("lit".to_string(), "gsfm not so lit".to_string()),
                        }
                    }
                    NestedMeta::Meta(_) => map.insert("nm nm".to_string(), "gsfm ffff".to_string()),
                };
            });
        },
        syn::Meta::NameValue(m) => { 
            let v = match &m.lit {
                Lit::Str(s) => s.value(),
                _ => "STRING".to_string(),
            };
            map.insert(m.path.get_ident().unwrap().to_string(), v);
        }
    };
    // dbg!(&map);
    map 
}

/// Check for an attribute proc_macro to set Domo Column type from or infer Domo Column type from the field's Rust type
/// if no domo attribute setting column_type is present.
fn get_domo_column_type(f: &Field) -> Result<String, syn::Error> {
    let at = &f.attrs;
    let domo_attr = at.iter().find(|a| a.path.is_ident("domo"));
    if let Some(d_attr) = domo_attr {
        let dv = d_attr.parse_meta()?;
        let span = dv.span();
        let v = match dv {
            syn::Meta::List(m) => { 
                let list: Vec<String> = m.nested.iter().map(|nm| {
                    let out = match nm {
                        NestedMeta::Lit(l) => {
                            match l {
                                Lit::Str(s) => s.value(),
                                // TODO: panic instead of implicitly going to "" or STRING?
                                _ => "".to_string(),
                            }
                        }
                        NestedMeta::Meta(m) => {
                            let map = get_string_from_meta(m);
                            // dbg!(&map);
                            map.get("column_type").unwrap_or(&"".to_string()).to_string()
                        }
                    };
                    out
                }).filter(|s| s != &"".to_string()).collect();
                // dbg!(&list);
                let n = list.first().map_or_else(|| get_domo_column_type_from_ident_ty(f), |v| v.to_string());
                n
            },
            syn::Meta::Path(m) => { format!("{:?}", m.type_id())},
            syn::Meta::NameValue(m) => { format!("{:?}", m.type_id())},
        };
        Ok(sanitize_domo_column_type(v, span)?)
    } else {
        let str_ty_val = get_domo_column_type_from_ident_ty(f);
        Ok(str_ty_val)
    }
}

/// Attempt to infer Domo Column type from the field's Rust type.
fn get_domo_column_type_from_ident_ty(f: &Field) -> String {
    let ty = &f.ty;
    let stv = match ty {
        syn::Type::Path(tp)  => match option_inner_type(&tp.path) {
            Some(inner_ty) => {
                inner_ty.into_token_stream().to_string() // this will give us something like "i32" for Option<i32>
            },
            None => {
                let tyi = tp.path.get_ident();
                if let Some(t) =tyi {
                    // dbg!(t);
                    t.to_string()
                } else {
                    // dbg!("get_domo_column_type syn::Type::Path None");
                    "".to_string()
                }
            }
        }
        _ => unimplemented!()
    };
    let str_ty_val = map_type_to_domo_type(stv);
    str_ty_val
}

/// Check to make sure the column_type value is a valid Domo Column Type.
fn sanitize_domo_column_type(raw: String, span: Span) -> Result<String, syn::Error> {
    match raw.as_str() {
        "LONG" => { Ok(raw) },
        "DOUBLE" => { Ok(raw) },
        "DECIMAL" => { Ok(raw) },
        "DATE" => { Ok(raw) },
        "DATETIME" => { Ok(raw) },
        "STRING" => { Ok(raw) },
        _ => {
            Err(syn::Error::new(span, format!("the value {} is not a recognized Domo Column Type.", raw)))
        }
    }
}

/// Check if the Path is an Option<_> and if so return Some(inner_type)
fn option_inner_type(path: &Path) -> Option<&Type> {
    if path.leading_colon.is_some() {
        return None;
    }

    if path.segments.len() != 1 || path.segments[0].ident != "Option" {
        return None;
    }

    let ab = match &path.segments[0].arguments {
        PathArguments::AngleBracketed(ab) => ab,
        _ => return None,
    };

    if ab.args.len() != 1 {
        return None;
    }

    match &ab.args[0] {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    }
}