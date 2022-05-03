# field_accessor

With this procedural macro, a user can dynamically get a field of a struct by a `String` type variable.
This program is currently experimental and I haven't written test codes yet.

## Usage

```
[dependencies]
field_accessor = {git = "https://github.com/europeanplaice/field_accessor"}
```

### Definition
This macro provides the two methods for structs by implementing `GetterSetter` trait. Using `get` you can get a field's value dynamically.
Also, a field's value can be updated by `set`.
```rust
trait GetterSetter<T> {
    fn get(&mut self, field_string: String) -> T;
    fn set(&mut self, field_string: String, value: T);
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
    dog.set(field_name.clone(), value_to_update);
    let fieldvalue: String = dog.get(field_name);
    println!("{:?}", fieldvalue);

    let field_name = "age".to_string();
    let value_to_update = 4u32;
    dog.set(field_name.clone(), value_to_update);
    let fieldvalue: u32 = dog.get(field_name);
    println!("{:?}", fieldvalue);

    let field_name = "life_expectancy".to_string();
    let value_to_update = 10u32;
    dog.set(field_name.clone(), value_to_update);
    let fieldvalue: u32 = dog.get(field_name);
    println!("{:?}", fieldvalue);

}


```
### output
```
"Jiro"
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
    fn set(&mut self, field_string: String, value: T);
    fn get(&mut self, field_string: String) -> T;
}
impl GetterSetter<String> for Dog {
    fn set(&mut self, field_string: String, value: String) {
        match &*field_string {
            "name" => self.name = value.clone(),
            _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["invalid field name"],
                &[],
            )),
        }
    }
    fn get(&mut self, field_string: String) -> String {
        match &*field_string {
            "name" => self.name.clone(),
            _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["invalid field name"],
                &[],
            )),
        }
    }
}
impl GetterSetter<u32> for Dog {
    fn set(&mut self, field_string: String, value: u32) {
        match &*field_string {
            "age" => self.age = value.clone(),
            "life_expectancy" => self.life_expectancy = value.clone(),
            _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["invalid field name"],
                &[],
            )),
        }
    }
    fn get(&mut self, field_string: String) -> u32 {
        match &*field_string {
            "age" => self.age.clone(),
            "life_expectancy" => self.life_expectancy.clone(),
            _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["invalid field name"],
                &[],
            )),
        }
    }
}
```

This code is generated at compiling.