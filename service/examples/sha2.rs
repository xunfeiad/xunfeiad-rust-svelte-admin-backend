use std::io::Read;

use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};
use base64::prelude::*;

fn main() {
    let mut hasher = Sha512::new();
    hasher.update(b"hello world");
    let res = hasher.finalize();

    let based = BASE64_STANDARD.encode(res);
    println!("{:?}",based)
}
