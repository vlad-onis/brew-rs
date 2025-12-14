use crate::auth::common::Email;

pub struct SignUpParams {
    pub email: Email,
    pub password: String,
}
