use axum::{extract::State, routing::get};
use std::sync::Arc;
use worker::{Env, Request, Response, Result};

use service::Query;

pub use axum;
pub use service;
pub use worker;
pub use worker_macros;

macro_rules! handler {
    ($name:path) => {
        |State(env): State<Arc<Env>>, req: axum::extract::Request| async {
            let resp = $name(req.try_into().expect("convert request"), env)
                .await
                .expect("handler result");
            Into::<axum::http::Response<axum::body::Body>>::into(resp)
        }
    };
}

pub fn router(env: Arc<Env>) -> axum::Router {
    axum::Router::new()
        .route("/api/asns", get(handler!(all_asn)))
        .with_state(env)
}

#[worker::send]
async fn all_asn(_: Request, env: Arc<Env>) -> Result<Response> {
    let asn_list = Query::query_all(env).await?;
    Response::builder().from_json(&asn_list)
}
