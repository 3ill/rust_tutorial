/**
* !Topic: Working with Expressions Demo
*/

enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

/**
 * !NOTE: when working with rust, you can assign expressions to variables
//? a function to grant access using the match expression
 */
fn grant_access(level: Access) {
    let can_access = level;
    let authorization = match can_access {
        Access::Admin => "Admin Detected: Root Privileges Granted",
        Access::Guest => "Guest Detected: Restricted Access Granted",
        Access::Manager => "Manager Detected: Executive privileges granted",
        Access::User => "User Detected:  Access Granted",
    };
    println!("{authorization}");
}

//? function to print message
fn print_message(standard: i32) {
    let is_gt_100 = standard > 100;
    match is_gt_100 {
        true => println!("value is greater than 100"),
        false => println!("Value is less than 100"),
    }
}

fn main() {
    grant_access(Access::Manager);
    grant_access(Access::Admin);
    grant_access(Access::Guest);
    grant_access(Access::User);
    print_message(150);
}
