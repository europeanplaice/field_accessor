# field_accessor

With this procedural macro, a user can dynamically get a field of a struct by a `String` type variable.
This program is currently experimental and I haven't written test codes yet.

## Installation

```
[dependencies]
field_accessor = {git = "https://github.com/europeanplaice/field_accessor"}
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

## Example
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