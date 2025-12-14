use tracing::info;

pub async fn sign_up() -> &'static str {
    info!("Called sign_up");

    "Sign up successful"
}
