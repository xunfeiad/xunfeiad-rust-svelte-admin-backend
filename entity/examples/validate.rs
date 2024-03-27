use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(url)]
    site: String,
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = 18, max = 20))]
    age: u32,
    #[validate(range(exclusive_min = 0.0, max = 100.0))]
    height: f32,
}

fn main() {
    let signup_data = SignupData {
        mail: "".to_string(),
        site: "".to_string(),
        first_name: "".to_string(),
        age: 0,
        height: 0.0,
    };
    match signup_data.validate() {
        Ok(_) => println!("Ok"),
        Err(e) => eprintln!("{:?}", e),
    }
}
