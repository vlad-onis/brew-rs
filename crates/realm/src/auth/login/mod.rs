pub mod login_token;

use thiserror::Error;
use tracing::{error, info};

use crate::{auth::login::login_token::generate_token, context::Context};

use super::password::{Error as HashingError, verify_password};
use brew_types::auth::login::LoginParams;
use storage::users::get_user_by_email;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Hashing error: {0}")]
    Hashing(#[from] HashingError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Password missmatch")]
    PasswordMissmatch,
    #[error("Email does not exist")]
    EmailDoesNotExist,
    #[error("Token generation error: {0}")]
    Token(#[from] login_token::Error),
}

/// Sign up a new user with the given email and password
pub async fn login_handler(params: LoginParams, context: Context) -> Result<String, Error> {
    let email = params.email;

    info!(?email, "Signing up user with email");

    let mut conn = context.db.pool.acquire().await?;

    info!(
        "Verifying if email already exists: {}",
        String::from(&email)
    );
    let Ok(user) = get_user_by_email(String::from(&email), &mut *conn).await else {
        error!("Email {:?} does not exist", email);
        return Err(Error::EmailDoesNotExist);
    };

    info!("Verifying password hash match");

    verify_password(params.password, user.password_hash)?;

    // todo: remove that unwrap and handle the error
    Ok(generate_token(user.id.unwrap())?)
}

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     use brew_types::auth::common::Email;

//     #[tokio::test]
//     pub async fn test_check_email_already_exists() {
//         let email = Email::new("test@example.com");
//         assert!(!check_email_already_exists(&email).await.unwrap());
//     }
// }
