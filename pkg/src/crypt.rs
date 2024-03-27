use crate::WebError;
use anyhow::{anyhow, Error, Result};
use base64::{prelude::BASE64_STANDARD, Engine};
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use sha2::{Digest, Sha384, Sha512};
use std::collections::BTreeMap;
use std::ops::{Add, Sub};
use std::time::{SystemTime, UNIX_EPOCH};
use syn::parse::Parser;

const SECRET: &'static str = "xunfei123";
const TIME_DELTA: u64 = 86400;

pub fn sha256_hash(str: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(str);
    hasher.update(SECRET);
    let encrpyed_password = hasher.finalize();
    BASE64_STANDARD.encode(BASE64_STANDARD.encode(encrpyed_password))
}

pub fn jwt_encrypt(id: usize) -> Result<String> {
    let key: Hmac<Sha384> = Hmac::new_from_slice(SECRET.as_bytes())?;
    let mut claims = BTreeMap::new();
    let time = SystemTime::from(UNIX_EPOCH)
        .elapsed()
        .expect("Fetch system time failed.");
    let sub_time = time.as_secs().add(TIME_DELTA);
    claims.insert("sub", sub_time.to_string());
    claims.insert("iat", id.to_string());
    Ok(claims.sign_with_key(&key)?)
}

pub fn validate_jwt(token: &str) -> Result<usize> {
    let key: Hmac<Sha384> = Hmac::new_from_slice(SECRET.as_bytes())?;
    let token: Token<Header, BTreeMap<String, String>, _> = token.verify_with_key(&key)?;
    let claims = token.claims();
    let time: u64 = claims
        .get("sub")
        .ok_or(anyhow!("get jwt inner sub failed."))?
        .parse()?;
    // .ok_or(Err(anyhow!("get jwt inner sub failed."))

    let current_time = SystemTime::from(UNIX_EPOCH).elapsed()?.as_secs();
    if time.lt(&current_time) {
        return Err(anyhow!("expired signature."));
    }
    let id: usize = claims
        .get("iat")
        .ok_or(anyhow!("get jwt inner sub failed."))?
        .parse()?;
    Ok(id)
}

#[cfg(test)]
mod test {
    use crate::crypt::validate_jwt;

    #[test]
    pub fn test_jwt() {
        let token: &str = "eyJhbGciOiJIUzM4NCJ9.eyJpYXQiOiIxMiIsInN1YiI6IjE3MTA5NTM0MzkifQ.O3fJcgPYnlgJDZM_AdG41PxNh-z7lv962wnjt0GqCq3-jqvvMhhMG2BXAQYQvcxZ";
        let id = validate_jwt(token).expect("parsing jwt failed.");
        assert_eq!(id, 12);
    }
}
