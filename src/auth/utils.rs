use super::user::AppUser;
use crate::errors::{AuthenticationError, HashingModuleError};
use bcrypt;
use jwt_simple::prelude::*;
use lazy_static::lazy_static;
use std::fs;

lazy_static! {
    pub static ref PRIVATE_PEM_FILE_CONTENT: String =
        fs::read_to_string("/Users/tarunw/.ssh/id_rsa")
            .unwrap()
            .parse()
            .expect("Failed to load private pem file");
    pub static ref PUBLIC_PEM_FILE_CONTENT: String =
        fs::read_to_string("/Users/tarunw/.ssh/id_rsa.pub")
            .unwrap()
            .parse()
            .expect("Failed to load pub key file");
}

/// Generates a hashed password from raw password given
pub fn generate_hashed_password(plain_text: String) -> Result<String, HashingModuleError> {
    match bcrypt::hash(plain_text, 10) {
        Ok(hashed_pass) => return Ok(hashed_pass),
        Err(e) => return Err(HashingModuleError(e.to_string())),
    };
}

/// Checks if raw password is same as encrypted password
pub fn verify_password(
    raw_pass: String,
    encrypted_pass: String,
) -> Result<bool, HashingModuleError> {
    match bcrypt::verify(raw_pass, &encrypted_pass) {
        Ok(status) => return Ok(status),
        Err(e) => return Err(HashingModuleError(e.to_string())),
    }
}

pub fn generate_jwt_token(user: AppUser) -> Result<String, AuthenticationError> {
    let key_pair = match RS384KeyPair::from_pem(&PRIVATE_PEM_FILE_CONTENT) {
        Ok(parsed_keypair) => parsed_keypair,
        Err(e) => return Err(AuthenticationError(e.to_string())),
    };

    let claims = Claims::create(Duration::from_hours(8))
        .with_issuer("IRAM")
        .with_subject("Iram Access");
    let token = key_pair.sign(claims);
    return token.map_err(|e| AuthenticationError(e.to_string()));
}

pub fn verify_jwt_token(user: AppUser, token: String) -> Result<bool, AuthenticationError> {
    let mut options = VerificationOptions::default();
    // Accept tokens that will only be valid in the future
    options.accept_future = true;
    // reject tokens if they were issued more than 1 hour ago
    options.max_validity = Some(Duration::from_hours(8));
    // reject tokens if they don't include an issuer from that set
    options.allowed_issuers = Some(HashSet::from_strings(&["IRAM"]));

    let public_key = match RS384PublicKey::from_pem(&PUBLIC_PEM_FILE_CONTENT) {
        Ok(parsed_keypair) => parsed_keypair,
        Err(e) => return Err(AuthenticationError(e.to_string())),
    };

    let claims = match public_key.verify_token::<NoCustomClaims>(&token, Some(options)) {
        Ok(claims) => claims,
        Err(e) => return Err(AuthenticationError(e.to_string())),
    };
    Ok(true)
}
