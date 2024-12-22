use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use entity::asn;
use pkgs::{Error, Json};
use std::sync::Arc;

#[async_trait::async_trait(?Send)]
pub trait ASNRepo {
    async fn query_all(&self) -> Result<Vec<asn::Model>, Error>;
    async fn bulk_upsert(&self, asns: Vec<asn::Model>) -> Result<(), Error>;
    async fn delete_expired(&self, expired_in: chrono::Duration) -> Result<u64, Error>;
}

pub struct ASNUsecase {
    repo: Arc<dyn ASNRepo>,
}

unsafe impl Send for ASNUsecase {}
unsafe impl Sync for ASNUsecase {}

impl ASNUsecase {
    pub fn new(repo: Arc<dyn ASNRepo>) -> Self {
        Self { repo }
    }

    pub async fn query_all(&self) -> Result<Response, Error> {
        let res = self.repo.query_all().await?;

        Ok(Json(res).with_status_code(StatusCode::OK).into_response())
    }

    pub async fn bulk_upsert(&self, asns: Vec<asn::Model>) -> Result<(), Error> {
        self.repo.bulk_upsert(asns).await
    }

    pub async fn delete_expired(&self, expired_in: chrono::Duration) -> Result<u64, Error> {
        self.repo.delete_expired(expired_in).await
    }
}
