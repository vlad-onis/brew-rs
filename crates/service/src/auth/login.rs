use axum::{
    body::Body,
    extract::{Json, State},
    http::{StatusCode, header},
    response::Response,
};
use serde_json::json;
use tracing::{debug, error};

use brew_types::auth::login::{LoginParams, http::LoginRequest};

use realm::{auth::login::login_handler, context::Context};

pub async fn login(State(context): State<Context>, Json(request): Json<LoginRequest>) -> Response {
    // todo: don't log PII in prod
    debug!("Called sign_up with body: {:?}", request);

    let params = LoginParams::from(request);

    let login_result = login_handler(params, context).await;

    // todo: remove that expect statement
    match login_result {
        Ok(token) => {
            let cookie_value = format!(
                // expires in 15 days
                // todo: change the hardcoded expiration
                "auth_token={}; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=1296000",
                token
            );
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json") // Inform client it's JSON
                .header(header::SET_COOKIE, cookie_value) // The Critical Header
                .body(Body::from(
                    json!(
                    {
                        "message": "Login Successful"
                    })
                    .to_string(),
                ))
                .expect("Failed to create the response")
        }
        Err(e) => {
            error!("Login failed with: {e}");
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!(
                    {
                        "message": "Login Failed"
                    })
                    .to_string(),
                ))
                .expect("Failed to create the response")
        }
    }
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
