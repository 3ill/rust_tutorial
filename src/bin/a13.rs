/**
 * !Topic: Vectors
 * ?Requirements: print 10, 20, "thirty", 40 in a loop
 * ?Print the total number of elements in a vector
 *
 * todo: use a vector to store 4 numbers
 * todo: Iterate through the vector using a for...in loop
 * todo: Determine wether to print the number or print "thirty" inside the loop
 * todo: use the .len() function to print the number of elements in a vector
 */

fn main() {
    //? use a vector to store 4 numbers
    let my_numbers = vec![10, 20, 30, 40];

    //? Determine wether to print the number or print "thirty" inside the loop
    for num in &my_numbers {
        match num {
            30 => println!("Thirty"),
            _ => println!("{:?}", num),
        }
    }

    //? use the .len() function to print the number of elements in a vector
    println!("list size => {:?}", &my_numbers.len());
}
