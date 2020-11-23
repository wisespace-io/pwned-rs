use pwned::api::*;
#[tokio::main(basic_scheduler)]
async fn main() {
    
    let pwned = PwnedBuilder::default()
        .user_agent("my_user_agent")
        .api_key(std::env::var("HIBP_API_KEY").expect("You need to give your HIBP API key as the HIBP_API_KEY environment variable"))
        .build().unwrap();

    match pwned.check_email("test@wisespace.io").await {
        Ok(answer) => {
            for breach in answer {
                println!("Service {:?}, breach date {:?} Domain: {:?}", breach.name, breach.breach_date, breach.domain);
            }
        },
        Err(e) => println!("Message: {}", e),
    }

}
