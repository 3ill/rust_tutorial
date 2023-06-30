//? function to set number
fn set_number(number: i32) -> i32 {
    number
}

//? function to check and compare number
fn check_number(response: i32) {
    let limit = 5;
    if response > limit {
        println!("> 5")
    } else if response == limit {
        println!("== 5")
    } else {
        println!("<5")
    }
}

//? function to check name
fn check_name(name: &str) -> &str {
    name
}

//? function to match name

fn main() {
    let my_name = check_name("Chike");
    match my_name {
        "Thrill" => println!("That's my name"),
        "Chike" => println!("maybe"),
        "Alice" => println!("ofcourse not"),
        _ => println!("Absolutely not"),
    }
    let reply = set_number(6);
    check_number(reply);
}
