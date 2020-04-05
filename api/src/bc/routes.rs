use crate::bc::{Input, find_bc_service};
use actix_web::{post, web, HttpResponse, Responder};

#[post("/bc")]
async fn bc(input: web::Json<Input>) -> impl Responder {
    HttpResponse::Ok().json(find_bc_service(input.into_inner()))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(bc);
}