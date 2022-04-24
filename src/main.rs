use actix_web::http::StatusCode;
use actix_web::web::{Bytes, Data, Path};
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use qstring::QString;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::fs;
use std::sync::Arc;
use Condizionato::AppState;

#[get("/")]
async fn home(state: Data<Arc<AppState>>) -> impl Responder {
    let mut page = String::from(include_str!("../website/home.html"));
    let mut ac_units = state.units().clone();
    let mut cards = String::new();

    ac_units.shuffle(&mut thread_rng());

    for i in 0..3 {
        cards += &ac_units[i].into_card();
    }

    page = page.replace("${UNITS_CARDS}", &cards);

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(page)
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
async fn units(req: HttpRequest, state: Data<Arc<AppState>>) -> impl Responder {
    let query = QString::from(req.query_string());
    let mut units = state.units().clone();
    let mut page = String::from(include_str!("../website/units.html"));
    let mut replacement = String::new();

    if !query.is_empty() {
        let mut indexes = HashSet::new();
        if let Some(name) = query.get("name") {
            let name = name.replace("%20", " ");
            page = page.replace("${QUERY_VALUE}", &name);
            for i in 0..units.len() {
                if !units[i].name().contains(&name) {
                    indexes.insert(i);
                }
            }
        } else {
            page = page.replace("${QUERY_VALUE}", "");
        }

        let mut indexes: Vec<&usize> = indexes.iter().collect();
        indexes.sort();
        while !indexes.is_empty() {
            units.remove(*indexes.pop().unwrap());
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
    let state = Arc::new(state);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(home)
            .service(image_jpg)
            .service(image_svg)
            .service(units)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
