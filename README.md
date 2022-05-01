# struct_field_getter

With this macro, a user can get a field of a struct by a `String` type variable.

## usage

```
[dependencies]
struct_field_getter = {git = "https://github.com/europeanplaice/struct_field_getter"}
```

## example
```rust
use struct_field_getter::FieldGetter;

#[derive(FieldGetter)]
struct Dog {
    name: String,
    age: u32
}

fn main() {
    let v = "name".to_string();
    let mut a = Dog{name: "Taro".to_string(), age: 3};
    a = a.set(v, FieldEnum::name("Jiro".to_string()));
    let v = "name".to_string();
    let b = a.get(v);
    println!("{:?}", b);
}
```
### output
```
name("Jiro")
```

In this example, it returns `FieldEnum` enum.
```rust
enum FieldEnum {
    name(String),
    age(u32),
}
```
This macro generates `FieldEnum` inside the implementation which corresponds to the fields of the struct. This enables the getter and setter functions to accept arbitrary types. As a side effect, a user needs to give `FieldEnum` not a value itself to the setter function.
