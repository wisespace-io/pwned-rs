use pwned::api::*;

fn main() {
    let pwned = PwnedBuilder::default()
        .api_key(std::env::var("HIBP_API_KEY").expect("You need to give your HIBP API key as the HIBP_API_KEY environment variable"))
        .build().unwrap();

    match pwned.check_password("password") {
        Ok(pwd) => println!("Pwned? {} - Occurrences {}", pwd.found, pwd.count),
        Err(e) => println!("Error: {}", e),
    }
}