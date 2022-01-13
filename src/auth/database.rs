use std::error::Error;
use crate::errors::AuthenticationError;
use super::utils;
use crate::models::Users;
use crate::db::get_connection;
use crate::schema::users;
use crate::diesel::QueryDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use super::user::AppUser;

/// Used to add a user in database. Only admin users or users with permission 'add_user'
/// are allowed to add perform this action
pub fn add_user(_user: AppUser, password: String) -> Result<bool, Box<dyn Error>> {
    Ok(true)
}

/// Unauthenticated call, used for make first default user in database.
/// Username is static and password is requested from user. 
/// If default user already exists, API returns an error
pub fn add_default_user(password: String, email: String) -> Result<bool, Box<dyn Error>> {
    let default_admin_user: AppUser = AppUser {
        username: "sre-admin".to_string(),
        first_name: "SRE".to_string(),
        last_name: "Admin".to_string(),
        email: email,
        is_admin: true,
        permissions: Vec::new(),
        groups: Vec::new(),
        is_authenticated: false,
        is_active: true
    };
    add_user(default_admin_user, password)
}

/// This function is used to authenticate user with entered password.
pub fn authenticate(username: String, password: String) -> Result<AppUser, AuthenticationError> {
    let connection = get_connection().unwrap();

    let query_response = users::dsl::users
        .filter(users::dsl::username.eq(username.clone()))
        .load::<Users>(&connection)
        .expect("Error loading alert source");
    
    if query_response.len() == 0 {
        let err_msg = format!(
            "User with username - {} not found in database", username.clone()
        );
        log::warn!("{}", err_msg);
        return Err(AuthenticationError(err_msg));
    } else {
        let user = &query_response[0];
        match utils::verify_password(password, query_response[0].get_password()) {
            Ok(result) if result == true => return Ok(AppUser {
                username: user.username.to_string(),
                first_name: user.first_name.to_string(),
                last_name: user.last_name.to_string(),
                email: user.email.to_string(),
                is_active: user.is_active,
                is_authenticated: true,
                is_admin: user.is_admin,
                groups: Vec::new(),
                permissions: Vec::new()
            }),
            Ok(result) if result == false => return Err(AuthenticationError(format!("Password verification failed for user - {}", username))),
            Ok(_) => return Err(AuthenticationError("Unknown Error during password verification".to_string())),
            Err(e) => return Err(AuthenticationError(e.to_string()))
        };
    }
}

// /// This function checks if user is present in database
// pub fn get_user_details(username: String) -> Result<AppUser, UserNotFoundError> {

// }