/**
 * !Topic: Strings
 * ? Requirements: print out the names and favorite color of people age 10 and under
 * todo: use a struct for a persons age, name and fav color
 * todo: the color and name should be stored as a string
 * todo: create and store at least 3 people in the vector
 * todo: iterate through the vector using a for loop
 * todo: use an if expression to determine which name to be printed
 * todo: the names and color should be printed using a function
 */

//? use a struct for a persons age, name and fav color
struct Person {
    name: String,
    color: String,
    age: i32,
}

impl Person {
    fn create_person(_name: String, _color: String, _age: i32) -> Self {
        Self {
            name: _name,
            color: _color,
            age: _age,
        }
    }

    //? function to print name and color
    fn print_data(&self) {
        println!("Name: {:?}, Color: {:?}", self.name, self.color);
    }
}

fn main() {
    let person1 = Person::create_person("Thrill".to_owned(), "White".to_owned(), 23);
    let person2 = Person::create_person("Obinna".to_owned(), "Black".to_owned(), 26);
    let person3 = Person::create_person("Ikenna".to_owned(), "grey".to_owned(), 20);

    let people = vec![person1, person2, person3];

    for person in people {
        let min_age = 19;

        if person.age > min_age {
            Person::print_data(&person);
        }
    }
}
