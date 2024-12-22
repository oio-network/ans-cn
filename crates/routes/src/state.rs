use biz::ASNUsecase;

use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub uc: Arc<ASNUsecase>,
}
