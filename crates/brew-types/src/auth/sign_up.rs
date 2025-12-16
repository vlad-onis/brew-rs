use crate::auth::common::{Email, Password};

pub mod http {
    use serde::{Deserialize, Serialize};

    use super::*;

    /// Http request body for the sign up endpoint
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct SignUpRequest {
        pub email: String,
        pub password: String,
        pub first_name: String,
        pub last_name: String,
    }

    impl From<SignUpRequest> for SignUpParams {
        fn from(http_request: SignUpRequest) -> Self {
            // todo: implement a with_strength_check function on password to check the strength here. (returns a Result<Password, Error>)
            // todo: same as above but for email
            let password = Password::new(http_request.password);
            let email = Email::new(&http_request.email);

            SignUpParams {
                first_name: http_request.first_name,
                last_name: http_request.last_name,
                email,
                password,
            }
        }
    }
}

/// Parameters used by the business layer to create a new user account
pub struct SignUpParams {
    pub first_name: String,
    pub last_name: String,
    pub email: Email,
    pub password: Password,
}

pub struct SignUpResult;
