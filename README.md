# field_accessor

With this procedural macro, a user can dynamically get a field of a struct by a `String` type variable.
This program is currently experimental and I haven't written test codes yet.

## Usage

```
[dependencies]
field_accessor = {git = "https://github.com/europeanplaice/field_accessor"}
```

### Definition
This macro provides the two methods for structs. Using `get` you can get a field's value dynamically.
Also, a field's value can be updated by `set`.
```rust
pub fn get(self, field_string: String) -> FieldEnum;

pub fn set(&mut self, value: FieldEnum) -> ();
```

## What is `(struct name)FieldEnum`?
This macro generates `(struct name)FieldEnum` enum. For `Dog` struct,
```rust
enum DogFieldEnum {
    name(String),
    age(u32),
}
```
This macro generates `(struct name)FieldEnum` inside the implementation which corresponds to the fields of the struct. This enables the getter and setter functions to accept arbitrary types. As a side effect, a user needs to give it (not a value itself) to the setter function. However, `(struct name)FieldEnum` is defined by the macro so you don't have to care about it.
## Example
```rust
use field_accessor::FieldAccessor;
use rand::Rng;

#[derive(FieldAccessor)]
struct Dog {
    name: String,
    age: u32,
}

#[derive(FieldAccessor)]
struct Cat {
    name: String,
    age: u32,
    parents: (String, String)
}

fn main() {
    let mut rng = rand::thread_rng();
    let fieldname;
    let newvalue;

    let mut dog = Dog {
        name: "Taro".to_string(),
        age: 3,
    };

    if rng.gen::<f64>() > 0.5 {
        fieldname = "name".to_string();
        newvalue = DogFieldEnum::name("Jiro".to_string())
    } else {
        fieldname = "age".to_string();
        newvalue = DogFieldEnum::age(4)
    }

    dog.set(newvalue);
    let fieldvalue = dog.get(fieldname);
    println!("{:?}", fieldvalue);

    let fieldname;
    let newvalue;
    let mut cat = Cat {
        name: "Taro".to_string(),
        age: 3,
        parents: ("Tom".to_string(), "Taylor".to_string()),
    };

    if rng.gen::<f64>() > 0.5 {
        fieldname = "parents".to_string();
        newvalue = CatFieldEnum::parents(("Paul".to_string(), "Ada".to_string()))
    } else {
        fieldname = "age".to_string();
        newvalue = CatFieldEnum::age(4)
    }

    cat.set(newvalue);
    let fieldvalue = cat.get(fieldname);
    println!("{:?}", fieldvalue);
}

```
### output
```
age(4) or name("Jiro")
age(4) or parents(("Paul", "Ada"))
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
enum DogFieldEnum {
    name(String),
    age(u32),
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
        }
    }
}
impl Dog {
    pub fn get(self, field_string: String) -> DogFieldEnum {
        match &*field_string {
            "name" => DogFieldEnum::name(self.name),
            "age" => DogFieldEnum::age(self.age),
            _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["invalid field name"],
                &[],
            )),
        }
    }
    pub fn set(&mut self, value: DogFieldEnum) -> () {
        match value {
            DogFieldEnum::name(v) => self.name = v,
            DogFieldEnum::age(v) => self.age = v,
            _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["invalid field value"],
                &[],
            )),
        }
    }
}
struct Cat {
    name: String,
    age: u32,
    parents: (String, String),
}
enum CatFieldEnum {
    name(String),
    age(u32),
    parents((String, String)),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for CatFieldEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&CatFieldEnum::name(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "name");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&CatFieldEnum::age(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "age");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&CatFieldEnum::parents(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "parents");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl Cat {
    pub fn get(self, field_string: String) -> CatFieldEnum {
        match &*field_string {
            "name" => CatFieldEnum::name(self.name),
            "age" => CatFieldEnum::age(self.age),
            "parents" => CatFieldEnum::parents(self.parents),
            _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["invalid field name"],
                &[],
            )),
        }
    }
    pub fn set(&mut self, value: CatFieldEnum) -> () {
        match value {
            CatFieldEnum::name(v) => self.name = v,
            CatFieldEnum::age(v) => self.age = v,
            CatFieldEnum::parents(v) => self.parents = v,
            _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["invalid field value"],
                &[],
            )),
        }
    }
}
```

This code is generated at compiling.