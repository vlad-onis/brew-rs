use crate::auth::common::{Email, Password};

pub struct SignUpParams {
    pub first_name: String,
    pub last_name: String,
    pub email: Email,
    pub password: Password,
}
