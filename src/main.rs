use actix_web::http::StatusCode;
use actix_web::web::{Bytes, Path};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::fs;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(include_str!("../website/home.html"))
}

#[get("/images/{image}")]
async fn get_image(image: Path<String>) -> impl Responder {
    let bytes =
        Bytes::from(fs::read_to_string(format!("website/images/{}", image.into_inner())).unwrap());
    HttpResponse::build(StatusCode::OK)
        .content_type("image/jpeg")
        .body(bytes)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
