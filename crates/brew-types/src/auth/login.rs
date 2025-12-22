use crate::auth::common::{Email, Password};

pub mod http {
    use serde::{Deserialize, Serialize};

    use super::*;

    /// Http request body for the login endpoint
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct LoginRequest {
        pub email: String,
        pub password: String,
    }

    impl From<LoginRequest> for LoginParams {
        fn from(http_request: LoginRequest) -> Self {
            let email = Email::new(&http_request.email);
            let password = Password::new(http_request.password);
            LoginParams { email, password }
        }
    }
}

/// Parameters used by the business layer to create a new user account
pub struct LoginParams {
    pub email: Email,
    pub password: Password,
}
