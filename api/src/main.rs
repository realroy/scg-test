use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod bc;
mod doscg;
mod xyz;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::new().finish())
            .service(index)
            .configure(doscg::init_routes)
            .configure(xyz::init_routes)
            .configure(bc::init_routes)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
