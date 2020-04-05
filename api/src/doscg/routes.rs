use crate::doscg::DOSCG;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/DOSCG")]
async fn doscg() -> impl Responder {
    HttpResponse::Ok().json(DOSCG { message: "DOSCG".to_string() })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(doscg);
}