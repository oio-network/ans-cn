use worker::*;
use tower_service::Service;
use crate::asn::routes::make_router;
use crate::asn::service::ASNsService;

mod asn;

fn log_request(req: &HttpRequest) {
    console_log!(
        r"{} [{}] {} {:?} {}",
        req.headers().get("CF-Connecting-IP").unwrap().to_str().unwrap_or_else(|_| "-"),
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

    let mut router = make_router(env);

    Ok(router.call(req).await?)
}

#[event(scheduled)]
pub async fn scheduled(_: ScheduledEvent, env: Env, _: ScheduleContext) {
    let srv = ASNsService::new(env);
    match srv.crawl_asn().await {
        Ok(asn_list) => {
            srv.delete_all_asn().await;
            srv.batch_create_asn(Vec::from_iter(asn_list)).await;
        }
        Err(e) => console_error!("Error: {:?}", e),
    }
}
