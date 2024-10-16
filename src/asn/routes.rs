use crate::asn::service::ASNsService;
use axum::{extract::State, routing::get};
use std::sync::Arc;
use worker::{Env, Request, Response, Result};

macro_rules! handler {
    ($name:path) => {
        |State(srv): State<Arc<ASNsService>>, req: axum::extract::Request| async {
            let resp = $name(req.try_into().expect("convert request"), srv)
                .await
                .expect("handler result");
            Into::<axum::http::Response<axum::body::Body>>::into(resp)
        }
    };
}

pub fn make_router(env: Env) -> axum::Router {
    let srv = Arc::new(ASNsService::new(env));
    axum::Router::new()
        .route("/api/asns", get(handler!(all_asn)))
        .with_state(srv)
}

#[worker::send]
async fn all_asn(_: Request, srv: Arc<ASNsService>) -> Result<Response> {
    let asn_list = srv.query_all_asn().await?;
    Response::builder().from_json(&asn_list)
}
