use std;
use reqwest;

error_chain::error_chain! {

    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
        ParseFloatError(std::num::ParseFloatError);
        InvalidHeaderError(reqwest::header::InvalidHeaderValue);
    }

}
