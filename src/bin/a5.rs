/**
 * !TITLE: DECISION MAKING WITH MATCH
 * todo: display it's true or it's false based on the value of a boolean variable
 */

//? function to set boolean value
fn set_bool(response: bool) -> bool {
    response
}

//? function to set integer
fn set_value(a: i32) -> i32 {
    a
}

//? function to run the match operation
fn run_match(result: i32) {
    match result {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("whatever"),
    }
}

fn main() {
    let my_bool = set_bool(true);
    match my_bool {
        true => println!("it's true"),
        false => println!("it's false"),
    }

    let value = set_value(6);
    run_match(value);
}
