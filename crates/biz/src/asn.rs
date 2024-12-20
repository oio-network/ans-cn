use ::entity::{asn, asn::Entity as ASN};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use pkgs::{Error, Json};
use serde::Serialize;
use std::sync::Arc;
use worker::Env;

#[async_trait::async_trait]
pub trait ASNRepo: Send + Sync {
    async fn query_all(&self, env: Arc<Env>) -> Result<Vec<asn::Model>, Error>;
}
