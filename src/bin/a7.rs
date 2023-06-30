/**
 * !Topic: Working with an enum
 * todo: print the name of a color to the terminal
 * todo: create an enum with color name as variants
 * todo: use a function to print out the color name
 */

//? create an enum
//? An enum is a custom type
enum Color {
    Blue,
    Green,
    Red,
}

//? function to match enum
fn match_enum(tint: Color) {
    match tint {
        Color::Blue => println!("you chose blue"),
        Color::Green => println!("you chose green"),
        Color::Red => println!("you chose red"),
    }
}

fn main() {
    match_enum(Color::Green)
}
