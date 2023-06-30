/**
 * !Topic: Organizing similar data using structs
 * todo: print the flavor of a drink and it's fluid ounces
 * todo: use an enum to create different flavors of drink s
 * todo: use a struct to store drink flavor and fluid ounces
 * todo: use a function ro print out the drink flavor and ounces
 * todo: use a match expression to print the drink flavor
 */
//? enum of drink flavors
enum Flavor {
    Cola,
    Energy,
}

//? Structure of drink
struct Drinks {
    flavor: Flavor,
    ounces: f64,
}

//? function to create and print drink
fn print_drink(flav: Flavor, ounc: f64) {
    let drink = Drinks {
        flavor: flav,
        ounces: ounc,
    };
    match drink.flavor {
        Flavor::Cola => println!("you chose cola"),
        Flavor::Energy => println!("you chose energy"),
    }
    println!("ounce => {:?}", drink.ounces);
}

fn main() {
    print_drink(Flavor::Energy, 75.0);
}
