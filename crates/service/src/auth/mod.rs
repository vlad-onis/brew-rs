pub mod sign_up;

use axum::{Router, routing::post};

use crate::auth::sign_up::sign_up;
use realm::context::Context;

pub fn routes(context: Context) -> Router {
    Router::new()
        .route("/sign_up", post(sign_up))
        .with_state(context)
}
