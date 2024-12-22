use std::sync::Arc;

use biz::{ASNUsecase, CrawlUsecase};
use crawler::Crawler;
use data::{d1, ASNRepo, CrawlRepo};
use routes::{router, AppState};
use tower_service::Service;
use worker::*;

pub(crate) const DB_NAMESPACE: &str = "DB";
pub(crate) const KV_NAMESPACE: &str = "kv";

fn log_request(req: &HttpRequest) {
    console_log!(
        r"{} [{}] {} {:?} {}",
        req.headers()
            .get("CF-Connecting-IP")
            .unwrap()
            .to_str()
            .unwrap_or("-"),
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

    let e = Arc::new(env);

    let kv = Arc::new(e.kv(KV_NAMESPACE)?);
    let d1 = Arc::new(d1(e.clone(), DB_NAMESPACE.to_string()).await.unwrap());

    let asn_repo = Arc::new(ASNRepo::new(kv, d1));
    let asn_uc = Arc::new(ASNUsecase::new(asn_repo));
    let state = AppState { uc: asn_uc };

    Ok(router(state).call(req).await?)
}

#[event(scheduled)]
pub async fn scheduled(_: ScheduledEvent, env: Env, _: ScheduleContext) {
    let e = Arc::new(env);

    let kv = Arc::new(e.kv(KV_NAMESPACE).unwrap());
    let d1 = Arc::new(d1(e.clone(), DB_NAMESPACE.to_string()).await.unwrap());

    let asn_repo = Arc::new(ASNRepo::new(kv, d1));
    let asn_uc = Arc::new(ASNUsecase::new(asn_repo));

    let crawler = Arc::new(Crawler);
    let crawler_repo = Arc::new(CrawlRepo::new(crawler));
    let crawler_uc = Arc::new(CrawlUsecase::new(crawler_repo));

    match crawler_uc.crawl("https://whois.ipip.net/iso/CN").await {
        Ok(asns) => asn_uc.bulk_upsert(asns).await.unwrap_or_else(|e| {
            console_error!("Error: {:?}", e);
        }),
        Err(e) => {
            console_error!("Error: {:?}", e)
        }
    }
}
