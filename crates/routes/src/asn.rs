use crate::state::AppState;
use axum::{extract::State, response::Response};
use pkgs::Error;

#[worker::send]
pub async fn get_all_asns(State(state): State<AppState>) -> Result<Response, Error> {
    state.uc.query_all().await
}
