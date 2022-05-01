use syn::{parse_macro_input, DeriveInput, FieldsNamed};
use quote::quote;
use proc_macro::{self, TokenStream};

#[proc_macro_derive(FieldGetter)]
pub fn get(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, data, ..} = parse_macro_input!(input);

    let output = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed{named, ..}) => {
                let idents = named.iter().map(|f| &f.ident);
                let idents2 = named.iter().map(|f| &f.ident);
                let tys2 = named.iter().map(|f| &f.ty).clone();
                quote!{
                    #[derive(Debug)]
                    enum ReturnValue{
                        #(#idents2(#tys2)),*
                    }

                    impl #ident {
                        pub fn get(self, field: String) -> ReturnValue{
                            match &*field {
                                #(stringify!(#idents) => {
                                    ReturnValue::#idents(self.#idents)
                                }),*
                                _ => panic!("invalid field name")
                            }
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