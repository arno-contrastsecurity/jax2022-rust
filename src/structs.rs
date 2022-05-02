

pub fn structs() {
    let mut person = Person {
        name: "Max Mustermann".to_string(),
        age: 54,
        gender: Some(Gender::Male),
    };

    person.set_name("abc".to_string());

    println!("{:#?}", person);
    println!("{}", person.is_volljaehrig());

    let b = person == Person::new("x", 5);

    let v = vec![1, 2, 3, 4, 5];
    let first_odd = v.iter().find(|el| **el % 2 == 1);
    println!("{:?}", first_odd);
}

#[derive(Debug, PartialEq, Eq)]
struct Person {
    name: String,
    age: u8,
    gender: Option<Gender>,
}
impl Person {
    fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
            gender: None
        }
    }

    fn is_volljaehrig(&self) -> bool {
        self.age >= 18
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Gender {
    Male, Female, Other
}
