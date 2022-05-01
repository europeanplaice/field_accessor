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
    let a = Dog{name: "Taro".to_string(), age: 3};
    let b = a.get(v);
    println!("{:?}", b);
}
```
### output
```
name("Taro")
```