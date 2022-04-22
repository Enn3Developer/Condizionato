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

#[get("/images/{image}.jpg")]
async fn get_image_jpg(image: Path<String>) -> impl Responder {
    let bytes = Bytes::from(
        fs::read_to_string(format!("website/images/{}.jpg", image.into_inner())).unwrap(),
    );
    HttpResponse::build(StatusCode::OK)
        .content_type("image/jpeg")
        .body(bytes)
}

#[get("/images/{image}.svg")]
async fn get_image_svg(image: Path<String>) -> impl Responder {
    let bytes = Bytes::from(
        fs::read_to_string(format!("website/images/{}.svg", image.into_inner())).unwrap(),
    );
    HttpResponse::build(StatusCode::OK)
        .content_type("image/svg+xml")
        .body(bytes)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_image_jpg)
            .service(get_image_svg)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
