# field_accessor

With this procedural macro, you can dynamically get a field of a struct by a `String` type variable.
This program is currently experimental.
This can be useful if you don't know which field you want when compiling.
## Installation

```
[dependencies]
field_accessor = "0"
```

## About this macro
This macro provides the two methods for structs by implementing `GetterSetter` trait. Using `get` you can get a field's value dynamically.
Also, a field's value can be updated by `set`. The functionality is similar to python's `getattr`, `setattr`.
```rust
trait GetterSetter<T> {
    fn get(&mut self, field_string: &String) -> Result<&T, String>;
    fn set(&mut self, field_string: &String, value: T) -> Result<(), String>;
}
```

## Usage and Example
```rust
use field_accessor::FieldAccessor;

#[derive(FieldAccessor)]
struct Dog {
    name: String,
    age: u32,
    life_expectancy: u32,
}

fn main() {
    let mut dog = Dog {
        name: "Taro".to_string(),
        age: 3,
        life_expectancy: 9,
    };

    let field_name = "name".to_string();
    let value_to_update = "Jiro".to_string();
    dog.set(&field_name, value_to_update).unwrap();
    let value_on_error;
    let fieldvalue: &String = match dog.get(&"invalid_field".to_string()) {
        Ok(value) => value,
        Err(_) => {value_on_error = "Ken".to_string(); &value_on_error},
    };
    println!("{:?}", fieldvalue);

    let field_name = "age".to_string();
    let value_to_update = 4u32;
    dog.set(&field_name, value_to_update).unwrap();
    let fieldvalue: &u32 = dog.get(&field_name).unwrap();
    println!("{:?}", fieldvalue);

    let field_name = "life_expectancy".to_string();
    let value_to_update = 10u32;
    dog.set(&field_name, value_to_update).unwrap();
    let fieldvalue: &u32 = dog.get(&field_name).unwrap();
    println!("{:?}", fieldvalue);

}

```
### output
```
"Ken"
4
10
```

## What this macro generates (in this example)
```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use field_accessor::FieldAccessor;
struct Dog {
    name: String,
    age: u32,
    life_expectancy: u32,
}
enum DogFieldEnum {
    name(String),
    age(u32),
    life_expectancy(u32),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for DogFieldEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&DogFieldEnum::name(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "name");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&DogFieldEnum::age(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "age");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&DogFieldEnum::life_expectancy(ref __self_0),) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "life_expectancy");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl ::core::marker::StructuralPartialEq for DogFieldEnum {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for DogFieldEnum {
    #[inline]
    fn eq(&self, other: &DogFieldEnum) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&DogFieldEnum::name(ref __self_0), &DogFieldEnum::name(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&DogFieldEnum::age(ref __self_0), &DogFieldEnum::age(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &DogFieldEnum::life_expectancy(ref __self_0),
                        &DogFieldEnum::life_expectancy(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &DogFieldEnum) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&DogFieldEnum::name(ref __self_0), &DogFieldEnum::name(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&DogFieldEnum::age(ref __self_0), &DogFieldEnum::age(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &DogFieldEnum::life_expectancy(ref __self_0),
                        &DogFieldEnum::life_expectancy(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialOrd for DogFieldEnum {
    #[inline]
    fn partial_cmp(&self, other: &DogFieldEnum) -> ::core::option::Option<::core::cmp::Ordering> {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&DogFieldEnum::name(ref __self_0), &DogFieldEnum::name(ref __arg_1_0)) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0), &(*__arg_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                    (&DogFieldEnum::age(ref __self_0), &DogFieldEnum::age(ref __arg_1_0)) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0), &(*__arg_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                    (
                        &DogFieldEnum::life_expectancy(ref __self_0),
                        &DogFieldEnum::life_expectancy(ref __arg_1_0),
                    ) => match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0), &(*__arg_1_0)) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        }
                        cmp => cmp,
                    },
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                ::core::cmp::PartialOrd::partial_cmp(&__self_vi, &__arg_1_vi)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for DogFieldEnum {
    #[inline]
    fn clone(&self) -> DogFieldEnum {
        match (&*self,) {
            (&DogFieldEnum::name(ref __self_0),) => {
                DogFieldEnum::name(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&DogFieldEnum::age(ref __self_0),) => {
                DogFieldEnum::age(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&DogFieldEnum::life_expectancy(ref __self_0),) => {
                DogFieldEnum::life_expectancy(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
trait GetterSetter<T> {
    fn get(&mut self, field_string: &String) -> Result<&T, String>;
    fn set(&mut self, field_string: &String, value: T) -> Result<(), String>;
}
impl GetterSetter<String> for Dog {
    fn get(&mut self, field_string: &String) -> Result<&String, String> {
        match &**field_string {
            "name" => Ok(&self.name),
            _ => Err({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["invalid field name to get \'", "\'"],
                    &[::core::fmt::ArgumentV1::new_display(&field_string)],
                ));
                res
            }),
        }
    }
    fn set(&mut self, field_string: &String, value: String) -> Result<(), String> {
        match &**field_string {
            "name" => {
                self.name = value;
                Ok(())
            }
            _ => Err({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["invalid field name to set \'", "\'"],
                    &[::core::fmt::ArgumentV1::new_display(&field_string)],
                ));
                res
            }),
        }
    }
}
impl GetterSetter<u32> for Dog {
    fn get(&mut self, field_string: &String) -> Result<&u32, String> {
        match &**field_string {
            "age" => Ok(&self.age),
            "life_expectancy" => Ok(&self.life_expectancy),
            _ => Err({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["invalid field name to get \'", "\'"],
                    &[::core::fmt::ArgumentV1::new_display(&field_string)],
                ));
                res
            }),
        }
    }
    fn set(&mut self, field_string: &String, value: u32) -> Result<(), String> {
        match &**field_string {
            "age" => {
                self.age = value;
                Ok(())
            }
            "life_expectancy" => {
                self.life_expectancy = value;
                Ok(())
            }
            _ => Err({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["invalid field name to set \'", "\'"],
                    &[::core::fmt::ArgumentV1::new_display(&field_string)],
                ));
                res
            }),
        }
    }
}
impl Dog {
    fn getenum(&mut self, field_string: &String) -> Result<DogFieldEnum, String> {
        match &**field_string {
            "name" => Ok(DogFieldEnum::name(self.name.clone())),
            "age" => Ok(DogFieldEnum::age(self.age.clone())),
            "life_expectancy" => Ok(DogFieldEnum::life_expectancy(self.life_expectancy.clone())),
            _ => Err({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["invalid field name to get \'", "\'"],
                    &[::core::fmt::ArgumentV1::new_display(&field_string)],
                ));
                res
            }),
        }
    }
}
```

This code is generated at compiling.

## Known issues

You need to specify the data type of the returned value. If it is not given,
the compiler cannot infer the type. This restriction reduces the convenience of using this macro.

```rust
#[derive(FieldAccessor)]
struct Dog {
    name: String,
    age: u32,
    life_expectancy: u32,
}

let mut dog = Dog {
    name: "Taro".to_string(),
    age: 3,
    life_expectancy: 9,
};
let fields = vec![
    "name".to_string(),
    "age".to_string(),
    "life_expectancy".to_string(),
]
for field_name in fields.into_iter(){
    let fieldvalue = dog.get(&field_name).unwrap();
};
```

This code raises an error.
```
let fieldvalue = dog.get(&field_name).unwrap();
    ----------       ^^^ cannot infer type for type parameter `T` declared on the trait `GetterSetter`
    |
    consider giving `fieldvalue` the explicit type `&T`, where the type parameter `T` is specified
```

A workaround is to replace `get` with `getenum`. This macro defines `(struct name)FieldEnum` inside for you like below.
```rust
enum DogFieldEnum {
    name(String),
    age(u32),
    life_expectancy(u32),
}
```
You can use this as a return type. With this enum you can get any field's value without concerning a field's type.
```rust
let mut dog = Dog {
    name: "Taro".to_string(),
    age: 3,
    life_expectancy: 9,
};
let fields = vec![
    "name".to_string(),
    "age".to_string(),
    "life_expectancy".to_string(),
];
let mut fieldvalues: Vec<DogFieldEnum> = vec![];
for field_name in fields.into_iter(){
    fieldvalues.push(dog.getenum(&field_name).unwrap());
};
assert_eq!(fieldvalues[0], DogFieldEnum::name("Taro".to_string()));
assert_eq!(fieldvalues[1], DogFieldEnum::age(3));
```