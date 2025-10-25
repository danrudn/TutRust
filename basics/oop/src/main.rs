fn main() {
    println!("=== RUST OOP KONZEPTE ===\n");

    structs_and_methods();
    traits_examples();
}

// class person (data + functions)
struct Person {
    name: String, // Private by default
    age: u32,
    email: String,
}
impl Person {
    // constructor
    pub fn new(name: String, age: u32, email: String) -> Self {
        // returs Person
        Person { name, age, email }
    }

    pub fn get_name(&self) -> &str {
        // return a slice string (instance method)
        return &self.name;
    }

    pub fn have_birthday(&mut self) {
        // &mut self: allow to modify the instance (instance method)
        self.age += 1;
        println!("{} ist jetzt {} Jahre alt!", self.name, self.age);
    }

    // takes ownership (just an example), after calling person.into_string() the objects is moved
    pub fn into_string(self) -> String {
        format!("{} ({} Jahre)", self.name, self.age)
    }

    // private method
    fn validate_email(&self) -> bool {
        self.email.contains('@')
    }

    // public method, calling private method
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && self.validate_email()
    }
}

fn structs_and_methods() {
    let mut person = Person::new("Alice".to_string(), 25, "alice@example.com".to_string());
    println!("Person: {:?}", person);
    println!("Name: {}", person.get_name());
    println!("Valid: {}", person.is_valid());
    person.have_birthday();
}

// traits Interfaces
trait Interface1 {
    fn get_name(&self) -> String; // default public
    fn func2(&self) {
        println!("Interface1 default impl");
    }
}

trait Interface2 {
    fn get_age(&self) -> u32;
    fn func2(&self) {
        println!("Interface2 default impl");
    }
}

struct Object1 {
    name: String,
}

struct Object2 {
    age: u32,
}

struct Object3 {
    age: u32,
    name: String,
}

impl Interface1 for Object1 {
    fn get_name(&self) -> String {
        return self.name;
    }
}

impl Interface2 for Object2 {
    fn get_age(&self) -> u32 {
        return self.age;
    }
}

impl Interface1 for Object3 {
    fn get_name(&self) -> String {
        return self.name;
    }
}
impl Interface2 for Object3 {
    fn get_age(&self) -> u32 {
        return self.age;
    }
}

// generic function which takes objects with Interface1 & Interface2
fn show_info<T: Interface1 + Interface2>(obj: &T) {
    println!("Info: {} is {} years old", obj.get_name(), obj.get_age());
}

fn traits_examples() {
    println!("--- TRAITS EXAMPLES ---");

    let obj1 = Object1 {
        name: "Alice".to_string(),
    };
    let obj2 = Object2 { age: 25 };
    let obj3 = Object3 {
        name: "Bob".to_string(),
        age: 30,
    };

    // obj1 has only Interface1
    println!("obj1 name: {}", obj1.get_name());
    obj1.func2(); // Interface1 default impl

    // obj1 has only Interface2
    println!("obj2 age: {}", obj2.get_age());
    obj2.func2(); // Interface2 default impl

    // obj3 has only Interface1 + Interface2
    println!("obj3 name: {}", obj3.get_name());
    println!("obj3 age: {}", obj3.get_age());
    show_info(&obj3); // this works only with obj3

    println!();
}
