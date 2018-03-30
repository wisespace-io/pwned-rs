[![Crates.io](https://img.shields.io/crates/v/pwned.svg)](https://crates.io/crates/pwned)
[![Build Status](https://travis-ci.org/wisespace-io/pwned-rs.png?branch=master)](https://travis-ci.org/wisespace-io/pwned-rs)
[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)

# pwned-rs

Check your passwords against [Have I been pwned?](https://haveibeenpwned.com/)

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
pwned = { git = "https://github.com/wisespace-io/pwned-rs.git" }
```

### Check a password against the API and see the number of occurrences

It uses the range API, so only the first 5 characters of a SHA1 hashed password are sent to Have I been pwned?

```rust
extern crate pwned;

use pwned::api::*;

fn main() {
    let pwned = PwnedBuilder::default().build().unwrap();

    match pwned.check_password("password") {
        Ok(pwd) => println!("Pwned? {} - Occurrences {}", pwd.found, pwd.count),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Check all breaches for an account

```rust
extern crate pwned;

use pwned::api::*;

fn main() {
    let pwned = PwnedBuilder::default().build().unwrap();

    match pwned.check_email("test@example.com") {
        Ok(answer) => {
            for breach in answer {
                println!("Service {:?}, breach date {:?}", breach.name, breach.breach_date);
            }
        },
        Err(e) => println!("Message: {}", e),
    }
}
```