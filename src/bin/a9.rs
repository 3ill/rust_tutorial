/**
 * !Topic: Data management using tuples
 * ? Requirements: print whether the y value of a coordinate is greater than 5, less than 5
 * ? or equal to 5
 *
 * todo: use a function that returns tuples
 * todo: destructure the tuples into two variables
 * todo: use else.. if statement to determine what to print
 */

//? function that returns tuple
fn get_coord(a: i32, b: i32) -> (i32, i32) {
    (a, b)
}

//? function that checks value
fn get_check(a: i32) {
    let max = 5;
    if a > max {
        println!("Greater than 5")
    } else if a < 5 {
        println!("Less than 5")
    } else if a == 5 {
        println!("Equal to 5")
    }
}
//? advanced destructure and check function
fn advanced(c: i32, d: i32) {
    let (x, y) = get_coord(c, d);
    println!("{:?} {:?}", x, y);
    get_check(x);
}

fn main() {
    advanced(6, 5);
}
