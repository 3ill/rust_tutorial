//? function to check age
fn ask_age(age: i32) -> i32 {
    age
}

//? function to validate age
fn validate(result: i32) {
    if result >= 21 {
        println!("Ok to purchase");
    } else {
        println!("Underage");
    }
}

//? function to check bool
fn check(answer: bool) -> bool {
    answer
}

//? function to validate bool
fn validate_bool(response: bool) {
    if response == true {
        println!("Hello")
    } else {
        println!("Goodbye")
    }
}

fn main() {
    let age = ask_age(5);
    validate(age);

    let reply = check(true);
    validate_bool(reply);
}
