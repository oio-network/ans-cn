mod asn;
mod state;

use axum::{routing::get, Router};
pub use state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/asns", get(asn::get_all_asns))
        .with_state(state)
}
