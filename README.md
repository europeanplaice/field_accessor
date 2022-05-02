# field_accessor

With this procedural macro, a user can get a field of a struct by a `String` type variable.
This program is currently experimental and I haven't written test codes yet.

## Usage

```
[dependencies]
field_accessor = {git = "https://github.com/europeanplaice/field_accessor"}
```

### Definition
```rust
pub fn get(self, field_string: String) -> FieldEnum;

pub fn set(&mut self, value: FieldEnum) -> ();
```
## What is `FieldEnum`?
This macro generates `FieldEnum` enum such as below.
```rust
enum FieldEnum {
    name(String),
    age(u32),
}
```
This macro generates `FieldEnum` inside the implementation which corresponds to the fields of the struct. This enables the getter and setter functions to accept arbitrary types. As a side effect, a user needs to give `FieldEnum` not a value itself to the setter function. However, `FieldEnum` is created by the macro, you don't have to define `FieldEnum`.
## Example
```rust
use field_accessor::FieldAccessor;

#[derive(FieldAccessor)]
struct Dog {
    name: String,
    age: u32
}

fn main() {
    let mut rng = rand::thread_rng();
    let fieldname;
    let newvalue;

    if rng.gen::<f64>() > 0.5 {
        fieldname = "name".to_string();
        newvalue = FieldEnum::name("Jiro".to_string())
    } else {
        fieldname = "age".to_string();
        newvalue = FieldEnum::age(4)
    }

    let mut a = Dog {
        name: "Taro".to_string(),
        age: 3,
    };
    a.set(newvalue);
    let b = a.get(fieldname);
    println!("{:?}", b);
}

```
### output
```
age(4) or name("Jiro")
```

## What this macro generates (in this example)
```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use field_accessor::FieldAccessor;
use rand::Rng;
struct Dog {
    name: String,
    age: u32,
}
enum FieldEnum {
    name(String),
    age(u32),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for FieldEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&FieldEnum::name(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "name");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&FieldEnum::age(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "age");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl Dog {
    pub fn get(self, field_string: String) -> FieldEnum {
        match &*field_string {
            "name" => FieldEnum::name(self.name),
            "age" => FieldEnum::age(self.age),
            _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["invalid field name"],
                &[],
            )),
        }
    }
    pub fn set(&mut self, value: FieldEnum) -> () {
        match value {
            FieldEnum::name(v) => self.name = v,
            FieldEnum::age(v) => self.age = v,
            _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["invalid field value"],
                &[],
            )),
        }
    }
}
```

This code is generated at compiling.