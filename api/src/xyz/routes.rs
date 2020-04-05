use crate::xyz::{Input, find_xyz_service};
use actix_web::{post, web, HttpResponse, Responder};

#[post("xyz")]
async fn xyz(input: web::Json<Input>) -> impl Responder {
    HttpResponse::Ok().json(
      find_xyz_service(input.into_inner().values)
    )
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(xyz);
}