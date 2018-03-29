extern crate pwned;

use pwned::api::*;

fn main() {
    let pwned = PwnedBuilder::default().build().unwrap();

    match pwned.check_password("password") {
        Ok(pwd) => println!("Pwned? {} - Occurrences {}", pwd.found, pwd.count),
        Err(e) => println!("Error: {}", e),
    }
}