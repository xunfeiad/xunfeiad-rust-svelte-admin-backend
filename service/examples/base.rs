use base64::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let base64 = BASE64_STANDARD.encode(b"\xFF\xEC\x20\x55\0");
    println!("{base64}");
    let time = SystemTime::from(UNIX_EPOCH).elapsed().unwrap();
    println!("{:?}", time);
}
