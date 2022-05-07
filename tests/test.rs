#[cfg(test)]
mod tests_simple_struct {
    use field_accessor::FieldAccessor;

    #[derive(FieldAccessor)]
    pub struct Dog {
        name: String,
        age: u32,
        life_expectancy: u32,
    }
    #[test]
    fn test_invalid_name() {
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
            Err(_) => {
                value_on_error = "Ken".to_string();
                &value_on_error
            }
        };
        assert_eq!(fieldvalue, &"Ken".to_string());
    }
    #[test]
    fn test_simple() {
        let mut dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
        };
        let field_name = "age".to_string();
        let value_to_update = 4u32;
        dog.set(&field_name, value_to_update).unwrap();
        let fieldvalue: &u32 = dog.get(&field_name).unwrap();
        assert_eq!(fieldvalue, &4u32);
    }

    #[test]
    fn test_error() {
        let dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
        };
        let field_name = "favorite_food".to_string();
        let fieldvalue: Result<&String, String> = dog.get(&field_name);
        assert!(fieldvalue.is_err());
    }

    #[test]
    fn test_iterate_with_enum() {
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
    }
}

#[cfg(test)]
mod tests_vector {
    use field_accessor::FieldAccessor;

    #[derive(FieldAccessor)]
    pub struct Dog {
        name: String,
        age: u32,
        life_expectancy: u32,
        friends: Vec<String>,
    }

    #[test]
    fn test_simple() {
        let mut dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
            friends: vec!["Mike".to_string(), "Nozomi".to_string()],
        };
        let field_name = "friends".to_string();
        let value_to_update = vec!["Makoto".to_string(), "Maya".to_string()];
        dog.set(&field_name, value_to_update).unwrap();
        let fieldvalue: &Vec<String> = dog.get(&field_name).unwrap();
        assert_eq!(fieldvalue, &vec!["Makoto".to_string(), "Maya".to_string()]);
    }
}

#[cfg(test)]
mod tests_getstructinfo{

    use field_accessor::FieldAccessor;

    #[derive(FieldAccessor)]
    pub struct Dog {
        name: String,
        age: u32,
        life_expectancy: u32,
    }

    #[test]
    fn test_tests_getstructinfo(){
        let mut dog = Dog {
            name: "Taro".to_string(),
            age: 3,
            life_expectancy: 9,
        };
        let info = dog.getstructinfo();
        let mut fieldvalues = vec![];
        for i in info.field_names.iter() {
            fieldvalues.push(dog.getenum(i).unwrap());
        }
        assert_eq!(fieldvalues[0], DogFieldEnum::name("Taro".to_string()));
        assert_eq!(fieldvalues[1], DogFieldEnum::age(3));
        assert_eq!(fieldvalues[2], DogFieldEnum::life_expectancy(9));
    }
}
