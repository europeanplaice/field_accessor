# field_accessor

With this macro, a user can get a field of a struct by a `String` type variable.
This program is currently experimental and I haven't written test codes yet.

## usage

```
[dependencies]
field_accessor = {git = "https://github.com/europeanplaice/field_accessor"}
```

## example
```rust
use field_accessor::FieldAccessor;

#[derive(FieldAccessor)]
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
