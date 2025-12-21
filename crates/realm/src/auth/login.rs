use thiserror::Error;
use tracing::{error, info};

use crate::context::Context;

use super::password::{Error as HashingError, hash_password};
use brew_types::auth::login::LoginParams;
use storage::users::{UserRow, get_user_by_email, insert_user};

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
}

/// Sign up a new user with the given email and password
pub async fn login_handler(params: LoginParams, context: Context) -> Result<(), Error> {
    let email = params.email;

    info!(?email, "Signing up user with email");

    let mut conn = context.db.pool.acquire().await?;

    let Ok(user) = get_user_by_email(String::from(&email), &mut *conn).await else {
        error!("Email {:?} does not exist", email);
        return Err(Error::EmailDoesNotExist);
    };

    let hashed_password = hash_password(params.password)?;

    if hashed_password != user.password_hash {
        error!("Password does not match");
        return Err(Error::PasswordMissmatch);
    }

    Ok(())
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
