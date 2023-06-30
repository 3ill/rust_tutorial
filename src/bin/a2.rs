//? subtraction function
fn sub(a: i32, b: i32) -> i32 {
    a - b
}

//? addition function
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//? multiplication function
fn mult(a: i32, b: i32) -> i32 {
    a * b
}

//? division function
fn div(a: i32, b: i32) -> i32 {
    b / a
}

//? remainder function
fn rm(a: i32, b: i32) -> i32 {
    a % b
}

fn display_result(result: i32) {
    println!("{result}");
}

fn main() {
    let sub_answer = sub(10, 8);
    let add_answer = add(10, 20);
    let mult_answer = mult(5, 5);
    let div_answer = div(2, 10);
    let rm_answer = rm(6, 4);

    display_result(add_answer);
    display_result(sub_answer);
    display_result(mult_answer);
    display_result(div_answer);
    display_result(rm_answer)
}
