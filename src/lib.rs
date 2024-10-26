use std::sync::Arc;

use api::router;
use axum;
use crawler::{Crawler, CrawlerConfig};
use service::Mutation;
use tower_service::Service;
use worker::*;

fn log_request(req: &HttpRequest) {
    console_log!(
        r"{} [{}] {} {:?} {}",
        req.headers()
            .get("CF-Connecting-IP")
            .unwrap()
            .to_str()
            .unwrap_or_else(|_| "-"),
        req.method(),
        req.uri().path(),
        req.version(),
        Date::now().to_string(),
    );
}

#[event(fetch)]
pub async fn fetch(
    req: HttpRequest,
    env: Env,
    _: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    log_request(&req);

    console_error_panic_hook::set_once();

    Ok(router(Arc::new(env)).call(req).await?)
}

#[event(scheduled)]
pub async fn scheduled(_: ScheduledEvent, env: Env, _: ScheduleContext) {
    match Crawler::new(CrawlerConfig::default())
        .asn("https://whois.ipip.net/iso/CN")
        .await
    {
        Ok(asns) => Mutation::bulk_upsert(Arc::new(env), asns)
            .await
            .unwrap_or_else(|e| {
                console_error!("Error: {:?}", e);
            }),
        Err(e) => {
            console_error!("Error: {:?}", e)
        }
    }
}
