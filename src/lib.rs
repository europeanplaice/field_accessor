use proc_macro::{self, TokenStream};
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

/// # field_accessor
/// 
/// <img src="img/definition.gif" width="55%">
/// 
/// With this procedural macro, you can dynamically get and update a field of the struct by a `String` type variable.
/// It can be good for you if you don't know which field you want when compiling. The functionality is similar to python's `getattr`, `setattr`.
/// ## Installation
/// 
/// ```
/// [dependencies]
/// field_accessor = "0"
/// ```
/// 
/// ## About this macro
/// This macro provides the four methods for structs. Only for `get`, `set`, to deal with different types of each field, I defined `GetterSetter<T>` trait and implemented it for each type.
/// 
/// ```rust
/// trait GetterSetter<T> {
///     fn get(&self, field_string: &String) -> Result<&T, String>;
///     fn set(&mut self, field_string: &String, value: T) -> Result<(), String>;
/// }
/// 
/// //implement for each type
/// impl GetterSetter<String> for StructName {
///     fn get(&self, field_string: &String) -> Result<&String, String>;
///     fn set(&mut self, field_string: &String, value: String) -> Result<(), String>;
/// }
/// impl GetterSetter<u32> for StructName {
///     fn get(&self, field_string: &String) -> Result<&u32, String>;
///     fn set(&mut self, field_string: &String, value: u32) -> Result<(), String>;
/// }
/// etc...
/// ```
/// 
/// ### `get`
/// ```rust
/// fn get(&self, field_string: &String) -> Result<&T, String>;
/// ```
/// It returns a field's value. Note that you need to specify the return type.
/// ### `get_mut`
/// ```rust
/// fn get_mut(&mut self, field_string: &String) -> Result<&mut T, String>;
/// ```
/// Returns a mutable reference to the field corresponding to the field_string.
/// ### `set`
/// ```rust
/// fn set(&mut self, field_string: &String, value: String) -> Result<(), String>;
/// ```
/// It updates a field's value.
/// ### `take`
/// ```rust
/// fn take(&mut self, field_string: &String) -> Result<T, String>;
/// ```
/// Replaces a field's value with the default value of T, returning the previous field's value.
/// ### `swap`
/// ```rust
/// fn swap(&mut self, field_string: &String, field_string_y: &String) -> Result<(), String>;
/// ```
/// Swaps the values at two fields, without deinitializing either one.
/// ### `replace`
/// ```rust
/// fn replace(&mut self, field_string: &String, src: T) -> Result<T, String>;
/// ```
/// Moves src into the field, returning the previous field's value.
/// ### `getenum`
/// ```rust
/// fn getenum(&self, field_string: &String) -> Result<(StructName)FieldEnum, String>;
/// ```
/// It returns a field's value like as `get` method, but the return type is enum. This method is helpful when field types vary. I will explain about enum later.
/// 
/// ### `getstructinfo`
/// ```rust
/// fn getstructinfo(&self) -> (StructName)StructInfo;
/// ```
/// You can extract a struct's field names, types, and a struct name.
/// 
/// ## Usage and Example
/// ![run](img/run.gif)
/// 
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
/// ```
/// "Ken"
/// 4
/// 10
/// ```
/// 
/// This code is generated at compiling.
/// 
/// ## Known issues
/// 
/// You need to specify the data type of the returned value. If it is not given,
/// the compiler cannot infer the type. This restriction reduces the convenience of using this macro.
/// 
/// ```rust
/// #[derive(FieldAccessor)]
/// struct Dog {
///     name: String,
///     age: u32,
///     life_expectancy: u32,
/// }
/// 
/// let mut dog = Dog {
///     name: "Taro".to_string(),
///     age: 3,
///     life_expectancy: 9,
/// };
/// let fields = vec![
///     "name".to_string(),
///     "age".to_string(),
///     "life_expectancy".to_string(),
/// ]
/// for field_name in fields.into_iter(){
///     let fieldvalue = dog.get(&field_name).unwrap();
/// };
/// ```
/// 
/// This code raises an error.
/// ```
/// let fieldvalue = dog.get(&field_name).unwrap();
///     ----------       ^^^ cannot infer type for type parameter `T` declared on the trait `GetterSetter`
///     |
///     consider giving `fieldvalue` the explicit type `&T`, where the type parameter `T` is specified
/// ```
/// 
/// A workaround is to replace `get` with `getenum`. This macro defines `(struct name)FieldEnum` behind the scenes for you like below.
/// ```rust
/// enum DogFieldEnum {
///     name(String),
///     age(u32),
///     life_expectancy(u32),
/// }
/// ```
/// You can use this as a return type. With this enum you can get any field's value without concerning a field's type.
/// ```rust
/// let mut dog = Dog {
///     name: "Taro".to_string(),
///     age: 3,
///     life_expectancy: 9,
/// };
/// let fields = vec![
///     "name".to_string(),
///     "age".to_string(),
///     "life_expectancy".to_string(),
/// ];
/// let mut fieldvalues: Vec<DogFieldEnum> = vec![];
/// for field_name in fields.into_iter(){
///     fieldvalues.push(dog.getenum(&field_name).unwrap());
/// };
/// assert_eq!(fieldvalues[0], DogFieldEnum::name("Taro".to_string()));
/// assert_eq!(fieldvalues[1], DogFieldEnum::age(3));
/// ```
/// 
/// ## Getting struct's information
/// You can get the information of the struct with `(field name)StructInfo` by calling `getstructinfo`.
/// 
/// ### Definition of `(field name)StructInfo`
/// ```rust
/// struct DogStructInfo {
///     field_names: Vec<String>,
///     field_types: Vec<String>,
///     struct_name: String
/// }
/// ```
/// ### Example
/// ```rust
/// let info = dog.getstructinfo();
/// println!("{:?}", info);
/// for i in info.field_names.iter() {
///     let fieldvalue: DogFieldEnum = dog.getenum(i).unwrap();
///     println!("{:?}", fieldvalue);
/// }
/// ```
/// #### output
/// ```
/// DogStructInfo { field_names: ["name", "age", "life_expectancy"], field_types: ["String", "u32", "u32"], struct_name: "Dog" }
/// 
/// name("Jiro")
/// age(4)
/// life_expectancy(10)
/// ```
/// 
/// ## Author
/// Tomohiro Endo (europeanplaice@gmail.com)

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

                let field_idents = named
                    .iter()
                    .map(|f| &f.ident);

                let mut swap_ident = vec![];
                let mut swap_ident2 = vec![];
                for (outer_ident, outer_type) in named.iter().map(|f| &f.ident).zip(named.iter().map(|f| &f.ty)){
                    for (inner_ident, inner_type) in named.iter().map(|f| &f.ident).zip(named.iter().map(|f| &f.ty)){
                        if outer_type == inner_type {
                            if outer_ident != inner_ident{
                                swap_tys.push(inner_type);
                                swap_ident.push(outer_ident.clone());
                                swap_ident2.push(inner_ident.clone());
                            }
                        }
                    }
                }

                for name in named.clone().iter() {
                    if get_tys.contains(&name.ty) {
                    } else {
                        get_tys.push(name.ty.clone());
                        get_mut_tys.push(name.ty.clone());
                        take_tys.push(name.ty.clone());
                        replace_tys.push(name.ty.clone());
                        
                        set_tys.push(name.ty.clone());
                        let get_filtered_ident = named
                            .iter()
                            .filter(|x| x.ty == name.ty)
                            .map(|f| &f.ident);
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
                                    Ok(mem::take(&mut self.#take_filtered_ident))
                                }
                            ),*
                        });
                        replace_quotes.push(quote! {
                            #(
                                stringify!(#replace_filtered_ident) => {
                                    Ok(mem::replace(&mut self.#replace_filtered_ident, src))
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

                    use std::mem;

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
                        fn get_mut(&mut self, field_string: &String) -> Result<&mut T, String>;
                        fn take(&mut self, field_string: &String) -> Result<T, String>;
                        fn replace(&mut self, field_string: &String, src: T) -> Result<T, String>;
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
                            fn get_mut(&mut self, field_string: &String) -> Result<&mut #get_tys, String> {
                                match &**field_string {
                                    #get_mut_quotes,
                                    _ => Err(format!("invalid field name to get '{}'", field_string)),
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
                                        mem::swap::<#swap_tys>(&mut self.#swap_ident, &mut self.#swap_ident2);
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
            },
            syn::Fields::Unnamed(_) => panic!("Only NamedFields is supported"),
            syn::Fields::Unit => panic!("Only NamedFields is supported"),
        },
        syn::Data::Enum(_) => panic!("Enum is not supported. Only struct is supported"),
        syn::Data::Union(_) => panic!("Union is not supported. Only struct is supported"),
    };
    output.into()
}
