use std::{any::{Any, TypeId}, collections::HashMap};

use proc_macro2::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Field, Fields, Lit, Meta, NestedMeta, Type, TypeReference};
use quote::quote;

use domo_pitchfork::domo::dataset::{DomoSchema, Schema, Column, DomoDataType};

pub fn expand_dataset_schema(input: DeriveInput) -> TokenStream {
    let st_name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = match input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), ..}) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let columns = fields.into_iter().map(|f| {
        // let field_name = f.ident;
        // let return_ty = match f.ty {
        //     // shared references can simply be copied
        //     Type::Reference(r @ TypeReference { mutability: None, .. }) => quote! { #r },
        //     // fallback to adding a reference
        //     ty => quote! { &#ty },
        // };
        let name = get_domo_field_name(&f);
        let column_type = get_domo_field_type2(&f);
        quote! {
            Column {
                name: #name.to_owned(),
                column_type: #column_type.to_owned(),
            }
        }
    });

    quote! {
        #[automatically_derived]
        impl #impl_generics #st_name #ty_generics #where_clause {
            pub fn domo_dataset_schema() -> domo_pitchfork::domo::dataset::Schema {
                let columns = vec![
                    #(#columns),*
                ];
                domo_pitchfork::domo::dataset::Schema {
                    columns,
                }
            }
        }
    }
}


fn get_domo_field_type2(f: &Field) -> String {
    let at = &f.attrs;
    // dbg!(at);
    let domo_attr = at.iter().find(|a| a.path.is_ident("domo"));
    if let Some(d_attr) = domo_attr {
        let dv = d_attr.parse_meta().unwrap();
        // let attr_map: HashMap<String,String> = HashMap::new();
        let v = match dv {
            syn::Meta::Path(m) => { format!("{:?}", m.type_id())},
            syn::Meta::List(m) => { 
                let list: Vec<String> = m.nested.iter().map(|nm| {
                    let out = match nm {
                        NestedMeta::Lit(l) => {
                            match l {
                                Lit::Str(s) => s.value(),
                                _ => "STRING".to_string(),
                            }
                        }
                        NestedMeta::Meta(m) => {
                            // dbg!(&m);
                            let map = get_string_from_meta(m);
                            dbg!(&map);
                            map.get("column_type").unwrap_or(&"".to_string()).to_string()
                        }
                    };
                    out
                }).filter(|s| s != &"".to_string()).collect();
                dbg!(&list);
                let n = list[0].to_string();
                n
            },
            syn::Meta::NameValue(m) => { format!("{:?}", m.type_id())},
        };
        v
    } else {

    let ty = &f.ty;
    let stv = match ty {
        syn::Type::Path(tp)  => {
            let tyi = tp.path.get_ident();
            if let Some(t) =tyi {
                t.to_string()
            } else {
                "".to_string()
            }
        },
        _ => unimplemented!()
    };
    // let str_ty_val = format!("{:?}", ty);
    let str_ty_val = map_type_to_domo_type(stv);
    str_ty_val
    }
}

fn get_domo_field_name(f: &Field) -> String {
    let at = &f.attrs;
    let domo_attr = at.iter().find(|a| a.path.is_ident("domo"));
    match domo_attr {
        Some(d_attr) => {
            let dv = d_attr.parse_meta().unwrap();
            // let attr_map: HashMap<String,String> = HashMap::new();
            let v = match dv {
                syn::Meta::Path(m) => { format!("{:?}", m.get_ident())},
                syn::Meta::List(m) => { 
                    let list: Vec<String> = m.nested.iter().map(|nm| {
                        let out = match nm {
                            NestedMeta::Lit(l) => {
                                match l {
                                    Lit::Str(s) => s.value(),
                                    _ => "STRING".to_string(),
                                }
                            }
                            NestedMeta::Meta(m) => {
                                let map = get_string_from_meta(m);
                                dbg!(&map);
                                map.get("name").unwrap_or(&"".to_string()).to_string()
                            }
                        };
                        out
                    }).filter(|s| s != &"".to_string()).collect();
                    dbg!(&list);
                    let n = list[0].to_string();
                    n
                },
                syn::Meta::NameValue(m) => { format!("{}", "TODO:m")},
            };
            v
        }
        None => {

        let n = &f.ident;
        // let str_ty_val = format!("{:?}", ty);
        let str_name_val = n.as_ref().unwrap().to_string();
        str_name_val
        }
    }
}

fn map_type_to_domo_type(s: String) -> String {
    match s.as_str() {
        "isize" => "LONG".to_string(),
        "usize" => "LONG".to_string(),
        "f64" => "DOUBLE".to_string(),
        _ => "STRING".to_string(),
    }
}

fn get_string_from_meta(m: &Meta) -> HashMap<String, String> {
    let mut map = HashMap::new();
    match m {
        syn::Meta::Path(m) => { 
            map.insert("smp".to_string(), format!("get_string_from_meta {:?}", m.type_id()));
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
                    NestedMeta::Meta(m) => map.insert("nm nm".to_string(), "gsfm ffff".to_string()),
                };
            });
        },
        syn::Meta::NameValue(m) => { 
            // format!("{:?}", m)},
            let v = match &m.lit {
                Lit::Str(s) => s.value(),
                _ => "STRING".to_string(),
            };
            map.insert(m.path.get_ident().unwrap().to_string(), v);
        }
    };
    dbg!(&map);
    map 
}

// fn get_domo_type(f: &Field) -> String {
//     let domo_type = match f.type_id() {
//         TypeId::of::<String>() => "STRING",
//         TypeId::of::<isize>() => "LONG",
//         TypeId::of::<usize>() | TypeId::of::<i8>() | TypeId::of::<i16>() | TypeId::of::<i32>() | TypeId::of::<i64>() | TypeId::of::<i128>() => "LONG", // This might need to be string if Domo's actually using i64 for schemas defined with their type "LONG"
//         TypeId::of::<f64>() => "DOUBLE",
//         TypeId::of::<f32>() => "DOUBLE",
//         _ => "STRING",
//     };
//     domo_type
// }