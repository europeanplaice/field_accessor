use proc_macro::{self, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

#[proc_macro_derive(FieldAccessor)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let output = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let idents_enum = named.iter().map(|f| &f.ident);
                let idents_getenum = idents_enum.clone();
                let tys_enum = named.iter().map(|f| &f.ty);
                let tys_for_structinfo = tys_enum.clone();
                let enumname = format_ident!("{}{}", ident, "FieldEnum");
                let structinfo = format_ident!("{}{}", ident, "StructInfo");
                let gettersetter = format_ident!("{}{}", ident, "GetterSetter");

                let mut get_quotes = vec![];
                let mut get_mut_quotes = vec![];
                let mut take_quotes = vec![];
                let mut replace_quotes = vec![];
                let mut set_quotes = vec![];
                let mut get_tys = vec![];
                let mut get_mut_tys = vec![];
                let mut take_tys = vec![];
                let mut replace_tys = vec![];
                let mut swap_tys = vec![];
                let mut set_tys = vec![];

                let field_idents = named.iter().map(|f| &f.ident);

                let mut swap_ident = vec![];
                let mut swap_ident2 = vec![];
                for (outer_ident, outer_type) in named
                    .iter()
                    .map(|f| &f.ident)
                    .zip(named.iter().map(|f| &f.ty))
                {
                    for (inner_ident, inner_type) in named
                        .iter()
                        .map(|f| &f.ident)
                        .zip(named.iter().map(|f| &f.ty))
                    {
                        if outer_type == inner_type {
                            if outer_ident != inner_ident {
                                swap_tys.push(inner_type);
                                swap_ident.push(outer_ident.clone());
                                swap_ident2.push(inner_ident.clone());
                            }
                        }
                    }
                }

                for name in named.clone().iter() {
                    if !get_tys.contains(&name.ty) {
                        get_tys.push(name.ty.clone());
                        get_mut_tys.push(name.ty.clone());
                        take_tys.push(name.ty.clone());
                        replace_tys.push(name.ty.clone());
                        set_tys.push(name.ty.clone());

                        let get_filtered_ident =
                            named.iter().filter(|x| x.ty == name.ty).map(|f| &f.ident);
                        let get_mut_filtered_ident = get_filtered_ident.clone();
                        let take_filtered_ident = get_filtered_ident.clone();
                        let replace_filtered_ident = get_filtered_ident.clone();
                        let set_filtered_ident = get_filtered_ident.clone();

                        get_quotes.push(quote! {
                            #(
                                stringify!(#get_filtered_ident) => {
                                    Ok(&self.#get_filtered_ident)
                                }
                            ),*
                        });
                        get_mut_quotes.push(quote! {
                            #(
                                stringify!(#get_mut_filtered_ident) => {
                                    Ok(&mut self.#get_mut_filtered_ident)
                                }
                            ),*
                        });
                        take_quotes.push(quote! {
                            #(
                                stringify!(#take_filtered_ident) => {
                                    Ok(std::mem::take(&mut self.#take_filtered_ident))
                                }
                            ),*
                        });
                        replace_quotes.push(quote! {
                            #(
                                stringify!(#replace_filtered_ident) => {
                                    Ok(std::mem::replace(&mut self.#replace_filtered_ident, src))
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
                        #(#idents_enum(#tys_enum)),*
                    }

                    trait #gettersetter<T> {
                        fn get(&self, field_string: &String) -> Result<&T, String>;
                        fn get_mut(&mut self, field_string: &String) -> Result<&mut T, String>;
                        fn take(&mut self, field_string: &String) -> Result<T, String>;
                        fn replace(&mut self, field_string: &String, src: T) -> Result<T, String>;
                        fn set(&mut self, field_string: &String, value: T) -> Result<(), String>;
                    }

                    #(
                        impl #gettersetter<#set_tys> for #ident {
                            fn get(&self, field_string: &String) -> Result<&#get_tys, String> {
                                match &**field_string {
                                    #get_quotes,
                                    _ => Err(format!("invalid field name to get '{}'", field_string)),
                                }
                            }
                            fn get_mut(&mut self, field_string: &String) -> Result<&mut #get_tys, String> {
                                match &**field_string {
                                    #get_mut_quotes,
                                    _ => Err(format!("invalid field name to get_mut '{}'", field_string)),
                                }
                            }
                            fn take(&mut self, field_string: &String) -> Result<#take_tys, String> {
                                match &**field_string {
                                    #take_quotes,
                                    _ => Err(format!("invalid field name to take '{}'", field_string)),
                                }
                            }
                            fn replace(&mut self, field_string: &String, src: #replace_tys) -> Result<#replace_tys, String> {
                                match &**field_string {
                                    #replace_quotes,
                                    _ => Err(format!("invalid field name to replace '{}'", field_string)),
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

                        fn swap(&mut self, field_string: &String, field_string_y: &String) -> Result<(), String> {
                            match (&**field_string, &**field_string_y) {
                                #(
                                    (stringify!(#swap_ident), stringify!(#swap_ident2)) => {
                                        std::mem::swap::<#swap_tys>(&mut self.#swap_ident, &mut self.#swap_ident2);
                                        Ok(())
                                    }
                                ),*
                                _ => Err(format!("invalid field name to swap")),
                            }
                        }

                        fn getenum(&self, field_string: &String) -> Result<#enumname, String> {
                            match &**field_string {
                                #(stringify!(#idents_getenum) => {
                                    Ok(#enumname::#idents_getenum(self.#idents_getenum.clone()))
                                }),*
                                _ => Err(format!("invalid field name to getenum '{}'", field_string)),
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
            syn::Fields::Unnamed(_) => panic!("Only NamedFields is supported"),
            syn::Fields::Unit => panic!("Only NamedFields is supported"),
        },
        syn::Data::Enum(_) => panic!("Enum is not supported. Only struct is supported"),
        syn::Data::Union(_) => panic!("Union is not supported. Only struct is supported"),
    };
    output.into()
}
