/**
 * !Topic: Looping using the loop statement
 * todo: Display 1 through 4 on the terminal
 * todo: Display a count down from 5 to 1 using while loops
 */

//? function to run loop
fn run_loop() {
    //? the mut keyword marks the variable as a changeable variable
    let mut num = 1;
    loop {
        println!("{num}");
        num = num + 1;
        if num == 5 {
            break;
        }
    }
    println!("The countdown is complete")
}

//? while loop function
fn do_loop() {
    let mut n = 5;
    while n >= 1 {
        println!("{n}");
        n = n - 1;
    }
    println!("Completed")
}

fn main() {
    do_loop();
    run_loop();
}
