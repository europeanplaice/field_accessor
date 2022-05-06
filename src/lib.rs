use proc_macro::{self, TokenStream};
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

/// # field_accessor
/// 
/// With this procedural macro, you can dynamically get and update a field of a struct by a `String` type variable.
/// This program is currently experimental.
/// This can be useful if you don't know which field you want when compiling.
/// ## Installation
/// 
/// ```ignore
/// [dependencies]
/// field_accessor = "0"
/// ```
/// 
/// ## About this macro
/// This macro provides the two methods for structs by implementing `GetterSetter` trait. Using `get` you can get a field's value dynamically.
/// Also, a field's value can be updated by `set`. The functionality is similar to python's `getattr`, `setattr`.
/// ```rust
/// trait GetterSetter<T> {
///     fn get(&mut self, field_string: &String) -> Result<&T, String>;
///     fn set(&mut self, field_string: &String, value: T) -> Result<(), String>;
/// }
/// ```
/// 
/// ## Usage and Example
/// ```rust
/// use field_accessor::FieldAccessor;
/// 
/// #[derive(FieldAccessor)]
/// struct Dog {
///     name: String,
///     age: u32,
///     life_expectancy: u32,
/// }
/// 
/// fn main() {
///     let mut dog = Dog {
///         name: "Taro".to_string(),
///         age: 3,
///         life_expectancy: 9,
///     };
/// 
///     let field_name = "name".to_string();
///     let value_to_update = "Jiro".to_string();
///     dog.set(&field_name, value_to_update).unwrap();
///     let value_on_error;
///     let fieldvalue: &String = match dog.get(&"invalid_field".to_string()) {
///         Ok(value) => value,
///         Err(_) => {value_on_error = "Ken".to_string(); &value_on_error},
///     };
///     println!("{:?}", fieldvalue);
/// 
///     let field_name = "age".to_string();
///     let value_to_update = 4u32;
///     dog.set(&field_name, value_to_update).unwrap();
///     let fieldvalue: &u32 = dog.get(&field_name).unwrap();
///     println!("{:?}", fieldvalue);
/// 
///     let field_name = "life_expectancy".to_string();
///     let value_to_update = 10u32;
///     dog.set(&field_name, value_to_update).unwrap();
///     let fieldvalue: &u32 = dog.get(&field_name).unwrap();
///     println!("{:?}", fieldvalue);
/// 
/// }
/// 
/// ```
/// ### output
/// ```ignore
/// "Ken"
/// 4
/// 10
/// ```
/// 
/// This code is generated at compiling.
#[proc_macro_derive(FieldAccessor)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let output = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let idents_enum = named.iter().map(|f| &f.ident);
                let idents_getenum = idents_enum.clone();
                let tys = named.iter().map(|f| &f.ty);
                let tys_for_structinfo = tys.clone();
                let enumname = format_ident!("{}{}", ident, "FieldEnum");
                let structinfo = format_ident!("{}{}", ident, "StructInfo");

                let mut set_quotes = vec![];
                let mut get_quotes = vec![];
                let mut set_tys = vec![];
                let mut get_tys = vec![];

                let field_idents = named
                    .iter()
                    .map(|f| &f.ident);

                for name in named.clone().iter() {
                    if get_tys.contains(&name.ty) {
                    } else {
                        get_tys.push(name.ty.clone());
                        set_tys.push(name.ty.clone());
                        let get_filtered_ident = named
                            .iter()
                            .filter(|x| x.ty == name.ty)
                            .map(|f| &f.ident);
                        let set_filtered_ident = get_filtered_ident.clone();
                        get_quotes.push(quote! {
                            #(
                                stringify!(#get_filtered_ident) => {
                                    Ok(&self.#get_filtered_ident)
                                }
                            ),*
                        });
                        set_quotes.push(quote! {
                            #(
                                stringify!(#set_filtered_ident) => {
                                    {self.#set_filtered_ident = value; Ok(())}
                                }
                            ),*
                        });
                    }
                }
                quote! {

                    #[derive(Debug, Clone)]
                    struct #structinfo {
                        field_names: Vec<String>,
                        field_types: Vec<String>,
                        struct_name: String
                    }

                    #[derive(Debug, PartialEq, PartialOrd, Clone)]
                    #[allow(non_camel_case_types)]
                    enum #enumname{
                        #(#idents_enum(#tys)),*
                    }

                    trait GetterSetter<T> {
                        fn get(&self, field_string: &String) -> Result<&T, String>;
                        fn set(&mut self, field_string: &String, value: T) -> Result<(), String>;
                    }
                    #(
                        impl GetterSetter<#set_tys> for #ident {
                            fn get(&self, field_string: &String) -> Result<&#get_tys, String> {
                                match &**field_string {
                                    #get_quotes,
                                    _ => Err(format!("invalid field name to get '{}'", field_string)),
                                }
                            }
                            fn set(&mut self, field_string: &String, value: #set_tys) -> Result<(), String>{
                                match &**field_string {
                                    #set_quotes,
                                    _ => Err(format!("invalid field name to set '{}'", field_string)),
                                }
                            }
                        }
                    )*
                    impl #ident {
                        fn getenum(&mut self, field_string: &String) -> Result<#enumname, String> {
                            match &**field_string {
                                #(stringify!(#idents_getenum) => {
                                    Ok(#enumname::#idents_getenum(self.#idents_getenum.clone()))
                                }),*
                                _ => Err(format!("invalid field name to get '{}'", field_string)),
                            }
                        }

                        fn getstructinfo(&self) -> #structinfo {
                            #structinfo {
                                field_names: vec![#(stringify!(#field_idents).to_string()),*],
                                field_types: vec![#(stringify!(#tys_for_structinfo).to_string()),*],
                                struct_name: stringify!(#ident).to_string()}
                        }
                    }
                }
            }
            _ => panic!("unsupported fields"),
        },
        _ => panic!("unsupported data"),
    };
    output.into()
}
