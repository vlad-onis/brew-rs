use axum::extract::Json;
use serde::{Deserialize, Serialize};
use tracing::debug;

use brew_types::auth::{
    common::{Email, Password},
    sign_up::SignUpParams,
};

use realm::auth::sign_up::sign_up_handler;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

pub async fn sign_up(Json(request): Json<SignUpRequest>) -> &'static str {
    // todo: don't log PII in prod
    debug!("Called sign_up with body: {:?}", request);

    let params = SignUpParams::from(request);

    // todo: error handling
    sign_up_handler(params).await.expect("sign up failed");

    "Sign up successful"
}

impl From<SignUpRequest> for SignUpParams {
    fn from(http_request: SignUpRequest) -> Self {
        SignUpParams {
            first_name: http_request.first_name,
            last_name: http_request.last_name,
            email: Email::new(&http_request.email),
            password: Password::new(http_request.password),
        }
    }
}
