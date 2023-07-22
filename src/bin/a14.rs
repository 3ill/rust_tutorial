//? when passing a string as a parameter, it must be borrowed
fn print_string(data: &str) {
    println!("{:?}", data);
}

fn main() {
    //? to create an owned string, .to_owned() method or String::from() can be used
    let owned_string = "Thrill Baby".to_owned();
    let another_owned = String::from("Thrill baby");

    print_string(&owned_string);
    print_string(&another_owned);
}
