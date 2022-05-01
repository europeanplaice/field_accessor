use syn::{parse_macro_input, DeriveInput, FieldsNamed};
use quote::quote;
use proc_macro::{self, TokenStream};

#[proc_macro_derive(FieldAccessor)]
pub fn get(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, data, ..} = parse_macro_input!(input);

    let output = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed{named, ..}) => {
                let idents = named.iter().map(|f| &f.ident);
                let idents2 = named.iter().map(|f| &f.ident);
                let idents3 = named.iter().map(|f| &f.ident);
                let tys2 = named.iter().map(|f| &f.ty).clone();
                quote!{
                    #[derive(Debug)]
                    enum FieldEnum{
                        #(#idents2(#tys2)),*
                    }

                    impl #ident {
                        pub fn get(self, field_string: String) -> FieldEnum{
                            match &*field_string {
                                #(stringify!(#idents) => {
                                    FieldEnum::#idents(self.#idents)
                                }),*
                                _ => panic!("invalid field name")
                            }
                        }

                        pub fn set(mut self, field: String, value: FieldEnum) -> Self{
                            match &*field {
                                #(stringify!(#idents3) => {
                                    self.#idents3 = match value {
                                        FieldEnum::#idents3(v) => v,
                                        _ => panic!("invalid field value")
                                    };
                                }),*
                                _ => panic!("invalid field name")
                            };
                            self
                        }
                    }
                }
            }, 
            _ => panic!("unsupported fields"),
        },
        _ => panic!("unsupported data"),
    };
    output.into()
}