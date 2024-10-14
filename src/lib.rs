use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;

mod asn;


pub async fn hello_world() -> &'static str {
    "Hello World!"
}

fn router() -> Router {
    Router::new()
        .route("/api/hello-world", get(hello_world))
}

fn log_request(req: &HttpRequest) {
    console_log!(
        "[{}] {} - {}",
        req.method(),
        req.uri().path(),
        Date::now().to_string()
    );
}

#[event(fetch)]
pub async fn fetch(
    req: HttpRequest,
    _: Env,
    _: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    log_request(&req);

    console_error_panic_hook::set_once();

    Ok(router().call(req).await?)
}

#[event(scheduled)]
pub async fn scheduled(_: ScheduledEvent, _: Env, _: ScheduleContext) {
    let srv = asn::service::ASNsService::new();
    match srv.get_asn().await {
        Ok(asn_list) => {
            for asn in &asn_list {
                console_log!("[AS{}]: {}", asn.number, asn.name);
            }
        }
        Err(e) => console_error!("Error: {:?}", e),
    }
}
