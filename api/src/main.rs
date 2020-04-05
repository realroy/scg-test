use actix_web::{App, HttpResponse, HttpServer, Responder, get};

mod doscg;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .configure(doscg::init_routes)
    })
        .bind("127.0.0.1:5000")?
        .run()
        .await
}