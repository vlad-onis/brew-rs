pub mod sign_up;

use axum::{Router, routing::get};

use crate::auth::sign_up::sign_up;

pub fn routes() -> Router {
    Router::new().route("/sign_up", get(sign_up))
}
