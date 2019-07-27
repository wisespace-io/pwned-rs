extern crate pwned;

use pwned::api::*;

fn main() {
    let pwned = PwnedBuilder::default().build().unwrap();


    match pwned.check_email("flavio@wisespace.io") {
        Ok(answer) => {
            for breach in answer {
                println!("Service {:?}, breach date {:?} Domain: {:?}", breach.name, breach.breach_date, breach.domain);
            }
        },
        Err(e) => println!("Message: {}", e),
    }

}
