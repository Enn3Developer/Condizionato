use actix_web::http::StatusCode;
use actix_web::web::{Bytes, Data, Path};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::fs;
use Condizionato::AppState;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(include_str!("../website/home.html"))
}

#[get("/images/{image}.jpg")]
async fn image_jpg(image: Path<String>) -> impl Responder {
    let bytes = Bytes::from(
        fs::read_to_string(format!("website/images/{}.jpg", image.into_inner())).unwrap(),
    );
    HttpResponse::build(StatusCode::OK)
        .content_type("image/jpeg")
        .body(bytes)
}

#[get("/images/{image}.svg")]
async fn image_svg(image: Path<String>) -> impl Responder {
    let bytes = Bytes::from(
        fs::read_to_string(format!("website/images/{}.svg", image.into_inner())).unwrap(),
    );
    HttpResponse::build(StatusCode::OK)
        .content_type("image/svg+xml")
        .body(bytes)
}

#[get("/units")]
async fn units(state: Data<AppState>) -> impl Responder {
    serde_json::to_string(state.units())
}

#[get("/units/query/name/{query}")]
async fn query_by_name(name: Path<String>, state: Data<AppState>) -> impl Responder {
    let mut units_found = vec![];
    let name = name.into_inner();

    for unit in state.units() {
        if unit.name().contains(&name) {
            units_found.push(unit);
        }
    }

    serde_json::to_string(&units_found)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state: AppState = toml::from_str(&fs::read_to_string("website/data/units.toml")?)?;
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(index)
            .service(image_jpg)
            .service(image_svg)
            .service(units)
            .service(query_by_name)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
