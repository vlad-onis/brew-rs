use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::debug;

use brew_types::auth::login::{LoginParams, http::LoginRequest};

use realm::{auth::login::login_handler, context::Context};

pub async fn login(
    State(context): State<Context>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    // todo: don't log PII in prod
    debug!("Called sign_up with body: {:?}", request);

    let params = LoginParams::from(request);

    let login_result = login_handler(params, context).await;

    let (status, body) = match login_result {
        Ok(_) => (StatusCode::OK, "login successful".to_string()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Login failed with error: {e}"),
        ),
    };

    (status, body)
}

#[cfg(test)]
pub mod tests {

    use brew_types::auth::common::Email;

    use super::*;

    #[test]
    fn login_parameters() {
        let json = r#"{
            "email": "test@example.com",
            "password": "password",
            "first_name": "John",
            "last_name": "Doe"
        }"#;

        let login_request = serde_json::from_str::<LoginRequest>(json).unwrap();
        let login_params = LoginParams::from(login_request);

        assert_eq!(login_params.email, Email::from("test@example.com"))
    }
}
