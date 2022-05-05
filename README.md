# field_accessor

With this procedural macro, you can dynamically get a field of a struct by a `String` type variable.
This program is currently experimental and I haven't written test codes yet.
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
    let fieldvalue: &String = match dog.get(&"invalid_name".to_string()) {
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