use pwned::api::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let pwned = PwnedBuilder::default().build().unwrap();

    match pwned.check_password("password").await {
        Ok(pwd) => println!("Pwned? {} - Occurrences {}", pwd.found, pwd.count),
        Err(e) => println!("Error: {}", e),
    }
}
