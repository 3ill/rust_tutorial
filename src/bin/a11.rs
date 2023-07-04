/**
 * !Topic: Ownership
 * ?Requirements: print out the quantity and id number of a grocery item
 * todo: use a struct for the grocery item
 * todo: use an i32 field for quantity and id number
 * todo: create a function to display the quantity
 * todo: create a function to display the id number
 */

//? Grocery Struct
struct Grocery {
    quantity: i32,
    id: i32,
}

//? function to set item
fn set_item(a: i32, b: i32) -> Grocery {
    Grocery { quantity: a, id: b }
}

//? function to display quantity
//? the "&" symbol is used to denote that the function is a borrowing a variable
fn display_quantity(item: &Grocery) {
    println!("Quantity => {:?}", item.quantity);
    if item.quantity > 20 {
        println!("This is a large item");
    } else {
        println!("This is a moderate item");
    }
}

//? function to display id
//? the "&" symbol is used to denote that the function is a borrowing a variable
fn display_id(item: &Grocery) {
    println!("item id => {:?}", item.id);
}

//? the main function is the Owner of the items variable
//? it is responsible for managing the memory
fn main() {
    let items = set_item(25, 2);

    display_quantity(&items);
    display_id(&items);
}
