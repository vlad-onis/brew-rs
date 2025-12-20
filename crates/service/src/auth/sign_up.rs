use axum::extract::{Json, State};
use tracing::debug;

use brew_types::auth::sign_up::{SignUpParams, http::SignUpRequest};

use realm::{auth::sign_up::sign_up_handler, context::Context};

pub async fn sign_up(
    State(context): State<Context>,
    Json(request): Json<SignUpRequest>,
) -> &'static str {
    // todo: don't log PII in prod
    debug!("Called sign_up with body: {:?}", request);

    let params = SignUpParams::from(request);

    // todo: error handling
    sign_up_handler(params, context)
        .await
        .expect("sign up failed");

    "Sign up successful"
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn test_sign_up_request_deserialization() {
        let json = r#"{
            "email": "test@example.com",
            "password": "password",
            "first_name": "John",
            "last_name": "Doe"
        }"#;

        let request: SignUpRequest = serde_json::from_str(json).unwrap();
        let params = SignUpParams::from(request);

        assert_eq!(params.email, Email::new("test@example.com"));
        assert_eq!(params.password, Password::new("password".to_string()));
        assert_eq!(params.first_name, "John");
        assert_eq!(params.last_name, "Doe");
    }
}
