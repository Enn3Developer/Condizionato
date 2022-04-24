use actix_web::http::StatusCode;
use actix_web::web::{Bytes, Data, Path};
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use qstring::QString;
use std::collections::HashSet;
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
    let bytes =
        Bytes::from(fs::read(format!("website/images/{}.jpg", image.into_inner())).unwrap());
    HttpResponse::build(StatusCode::OK)
        .content_type("image/jpeg")
        .body(bytes)
}

#[get("/images/{image}.svg")]
async fn image_svg(image: Path<String>) -> impl Responder {
    let bytes =
        Bytes::from(fs::read(format!("website/images/{}.svg", image.into_inner())).unwrap());
    HttpResponse::build(StatusCode::OK)
        .content_type("image/svg+xml")
        .body(bytes)
}

#[get("/units")]
async fn units(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let query = QString::from(req.query_string());
    let mut units = state.units().clone();
    let mut page = String::from(include_str!("../website/units.html"));
    let mut replacement = String::new();

    if !query.is_empty() {
        let mut indexes = HashSet::new();
        let mut offset = 0;
        if let Some(name) = query.get("name") {
            page = page.replace("${QUERY_VALUE}", name);
            for (i, unit) in units.iter().enumerate() {
                if !unit.name().contains(name) {
                    indexes.insert(i);
                }
            }
        } else {
            page = page.replace("${QUERY_VALUE}", "");
        }
        for i in indexes {
            units.remove(i - offset);
            offset += 1;
        }
    } else {
        page = page.replace("${QUERY_VALUE}", "");
    }

    for unit in units {
        replacement += &unit.into_card();
    }

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(page.replace("${UNITS_CARDS}", &replacement))
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
