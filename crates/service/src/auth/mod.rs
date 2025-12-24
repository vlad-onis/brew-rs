pub mod login;
pub mod sign_up;

use axum::{Router, routing::post};

use crate::auth::{login::login, sign_up::sign_up};
use realm::context::Context;

pub fn routes(context: Context) -> Router {
    Router::new()
        .route("/sign_up", post(sign_up))
        .route("/login", post(login))
        .with_state(context)
}
