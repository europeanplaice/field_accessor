use proc_macro::{self, TokenStream};
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

/// With this procedural macro, you can dynamically get and update a field of a struct by a `String` type variable.
#[proc_macro_derive(FieldAccessor)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let output = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let idents_enum = named.iter().map(|f| &f.ident);
                let idents_getenum = named.iter().map(|f| &f.ident);
                let tys = named.iter().map(|f| &f.ty).clone();
                let tys_for_structinfo = named.iter().map(|f| &f.ty).clone();
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
                        let set_filtered_ident = named
                            .iter()
                            .filter(|x| x.ty == name.ty)
                            .map(|f| &f.ident);
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

                    #[derive(Debug)]
                    struct #structinfo {
                        field_names: Vec<String>,
                        field_types: Vec<String>,
                        struct_name: String
                    }

                    #[derive(Debug, PartialEq, PartialOrd, Clone)]
                    enum #enumname{
                        #(#idents_enum(#tys)),*
                    }

                    trait GetterSetter<T> {
                        fn get(&mut self, field_string: &String) -> Result<&T, String>;
                        fn set(&mut self, field_string: &String, value: T) -> Result<(), String>;
                    }
                    #(
                        impl GetterSetter<#set_tys> for #ident {
                            fn get(&mut self, field_string: &String) -> Result<&#get_tys, String> {
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
