# field_accessor

<img src="img/definition.gif" width="55%">

With this procedural macro, you can dynamically get and update a field of the struct by a `String` type variable.
It can be good for you if you don't know which field you want when compiling. The functionality is similar to python's `getattr`, `setattr`.
## Installation

```
[dependencies]
field_accessor = "0"
```

## About this macro
This macro provides the four methods for structs. Only for `get`, `set`, to deal with different types of each field, I defined `GetterSetter<T>` trait and implemented it for each type.

```rust
trait GetterSetter<T> {
    fn get(&self, field_string: &String) -> Result<&T, String>;
    fn set(&mut self, field_string: &String, value: T) -> Result<(), String>;
}

//implement for each type
impl GetterSetter<String> for StructName {
    fn get(&self, field_string: &String) -> Result<&String, String>;
    fn set(&mut self, field_string: &String, value: String) -> Result<(), String>;
}
impl GetterSetter<u32> for StructName {
    fn get(&self, field_string: &String) -> Result<&u32, String>;
    fn set(&mut self, field_string: &String, value: u32) -> Result<(), String>;
}
etc...
```

### `get`
```rust
fn get(&self, field_string: &String) -> Result<&T, String>;
```
It returns a field's value. Note that you need to specify the return type.
### `set`
```rust
fn set(&mut self, field_string: &String, value: String) -> Result<(), String>;
```
It updates a field's value.
### `getenum`
```rust
fn getenum(&self, field_string: &String) -> Result<(StructName)FieldEnum, String>;
```
It returns a field's value like as `get` method, but the return type is enum. This method is helpful when field types vary. I will explain about enum later.

### `getstructinfo`
```rust
fn getstructinfo(&self) -> (StructName)StructInfo;
```
You can extract a struct's field names, types, and a struct name.

## Usage and Example
![run](img/run.gif)

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

A workaround is to replace `get` with `getenum`. This macro defines `(struct name)FieldEnum` behind the scenes for you like below.
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

## Getting struct's information
You can get the information of the struct with `(field name)StructInfo` by calling `getstructinfo`.

### Definition of `(field name)StructInfo`
```rust
struct DogStructInfo {
    field_names: Vec<String>,
    field_types: Vec<String>,
    struct_name: String
}
```
### Example
```rust
let info = dog.getstructinfo();
println!("{:?}", info);
for i in info.field_names.iter() {
    let fieldvalue: DogFieldEnum = dog.getenum(i).unwrap();
    println!("{:?}", fieldvalue);
}
```
#### output
```
DogStructInfo { field_names: ["name", "age", "life_expectancy"], field_types: ["String", "u32", "u32"], struct_name: "Dog" }

name("Jiro")
age(4)
life_expectancy(10)
```

## Author
Tomohiro Endo (europeanplaice@gmail.com)