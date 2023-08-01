use actix_identity::Identity;
use actix_web::{web, HttpResponse};

use super::error::ApiError;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ping").route(web::get().to(ping)));
}

async fn ping(
    _: Identity,
) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().finish())
}


