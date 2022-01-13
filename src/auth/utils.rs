use bcrypt;
use crate::errors::HashingModuleError;

/// Generates a hashed password from raw password given
pub fn generate_hashed_password(plain_text: String) -> Result<String, HashingModuleError> {
    match bcrypt::hash(plain_text, 10) {
        Ok(hashed_pass) => return Ok(hashed_pass),
        Err(e) => return Err(HashingModuleError(e.to_string()))
    };
}

/// Checks if raw password is same as encrypted password
pub fn verify_password(raw_pass: String, encrypted_pass: String) -> Result<bool, HashingModuleError> {
    match bcrypt::verify(raw_pass, &encrypted_pass) {
        Ok(status) => return Ok(status),
        Err(e) => return Err(HashingModuleError(e.to_string()))
    }
}
