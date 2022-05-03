use syn::{parse_macro_input, DeriveInput, FieldsNamed};
use quote::{quote};
use proc_macro::{self, TokenStream};

#[proc_macro_derive(FieldAccessor)]
pub fn get(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, data, ..} = parse_macro_input!(input);

    let output = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed{named, ..}) => {
                let newnamed = named.clone();
                let mut set_quotes = vec![];
                let mut get_quotes = vec![];
                let mut set_tys = vec![];
                let mut get_tys = vec![];
                for name in newnamed.clone().iter() {
                    if get_tys.contains(&name.ty) {

                    } else {
                        get_tys.push(name.ty.clone());
                        set_tys.push(name.ty.clone());
                        let get_filtered_ident = newnamed.iter().filter(|x| x.ty == name.ty).map(|f| &f.ident);
                        let set_filtered_ident = newnamed.iter().filter(|x| x.ty == name.ty).map(|f| &f.ident);
                        set_quotes.push(quote!{
                            #(
                                stringify!(#get_filtered_ident) => {
                                    self.#get_filtered_ident = value.clone()
                                }
                            ),*
                        });
                        get_quotes.push(quote!{
                            #(
                                stringify!(#set_filtered_ident) => {
                                    self.#set_filtered_ident.clone()
                                }
                            ),*
                        });
                    }
                }
                quote!{
                    trait GetterSetter<T> {
                        fn set(&mut self, field_string: String, value: T);
                        fn get(&mut self, field_string: String) -> T;
                    }
                    
                    #(
                        impl GetterSetter<#set_tys> for #ident {
                            fn set(&mut self, field_string: String, value: #set_tys){
                                match &*field_string {
                                    #set_quotes,
                                    _ => panic!("invalid field name")
                                }
                            }

                            fn get(&mut self, field_string: String) -> #get_tys {
                                match &*field_string {
                                    #get_quotes,
                                    _ => panic!("invalid field name")
                                }
                            }

                        }
                    )*
                }
            }, 
            _ => panic!("unsupported fields"),
        },
        _ => panic!("unsupported data"),
    };
    output.into()
}