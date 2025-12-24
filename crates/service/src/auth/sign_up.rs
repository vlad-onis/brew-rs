use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::debug;

use brew_types::auth::sign_up::{SignUpParams, http::SignUpRequest};

use realm::{auth::sign_up::sign_up_handler, context::Context};

pub async fn sign_up(
    State(context): State<Context>,
    Json(request): Json<SignUpRequest>,
) -> impl IntoResponse {
    // todo: don't log PII in prod
    debug!("Called sign_up with body: {:?}", request);

    let params = SignUpParams::from(request);

    // todo: error handling
    let sign_up_result = sign_up_handler(params, context).await;

    let (status, body) = match sign_up_result {
        Ok(_) => (StatusCode::OK, "Sign up successful".to_string()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Sign up  failed with error: {e}"),
        ),
    };

    (status, body)
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use brew_types::auth::common::{Email, Password};

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
