pub mod sign_up;

use axum::{Router, routing::post};

use crate::auth::sign_up::sign_up;

pub fn routes() -> Router {
    Router::new().route("/sign_up", post(sign_up))
}
