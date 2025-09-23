use field_accessor::FieldAccessor;

#[derive(FieldAccessor)]
struct Example {
    name: String,
    count: usize,
}

fn main() {
    let mut example = Example {
        name: "test".to_string(),
        count: 1,
    };

    example.set(&"name".to_string(), "updated".to_string()).unwrap();
    let current: &String = example.get(&"name".to_string()).unwrap();
    assert_eq!(current, "updated");
}
