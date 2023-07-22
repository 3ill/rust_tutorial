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

//? Struct to create person object
struct Person {
    name: String,
    age: i32,
    color: String,
}

//? function to create person object
fn add_person(_name: String, _age: i32, _color: String) -> Person {
    Person {
        name: _name,
        age: _age,
        color: _color,
    }
}

//? function to print string
fn print_string(_name: &str, _color: &str) {
    println!("name => {:?}, color => {:?}", _name, _color)
}

fn main() {
    //? function to create a person
    let person1 = add_person("thrill".to_owned(), 23, "white".to_owned());
    let person2 = add_person("obinna".to_owned(), 27, "black".to_owned());
    let person3 = add_person("ikenna".to_owned(), 20, "grey".to_owned());

    //? vector array of the Person Struct
    let people = vec![person1, person2, person3];

    //? for loop with if expression to determine which data to be printed
    for person in people {
        let min_age = 16;
        if person.age > min_age {
            print_string(&person.name, &person.color);
        }
    }
}
