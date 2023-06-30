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

fn main() {
    grant_access(Access::Manager);
    grant_access(Access::Admin);
    grant_access(Access::Guest);
    grant_access(Access::User);
}
