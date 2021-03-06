#![feature(iter_advance_by)]

use actix_web::http::StatusCode;
use actix_web::web::{Bytes, Data, Path};
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use qstring::QString;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use std::collections::HashSet;
use std::fs;
use std::sync::Arc;
use Condizionato::AppState;

#[get("/")]
async fn home(state: Data<Arc<AppState>>) -> impl Responder {
    let page = include_str!("../website/home.html");
    let ac_units = state.units();
    let mut cards = String::new();

    let mut rng = thread_rng();
    let uniform = Uniform::new(0, ac_units.len());

    for _ in 0..=2 {
        let i = uniform.sample(&mut rng);
        cards += &ac_units[i].card(i);
    }

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(page.replace("${UNITS_CARDS}", &cards))
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
    let units = state.units();
    let mut indexes = HashSet::new();
    let mut page = String::from(include_str!("../website/units.html"));
    let mut replacement = String::new();

    if !query.is_empty() {
        if let Some(name) = query.get("name") {
            let name = name.replace("%20", " ");
            page = page.replace("${QUERY_VALUE}", &name);
            for i in 0..units.len() {
                if units[i].name().contains(&name) {
                    indexes.insert(i);
                }
            }
        } else {
            page = page.replace("${QUERY_VALUE}", "");
        }
    } else {
        page = page.replace("${QUERY_VALUE}", "");
    }

    if !indexes.is_empty() {
        for i in indexes {
            replacement += &units[i].card(i);
        }
    } else {
        for (i, unit) in units.iter().enumerate() {
            replacement += &unit.card(i);
        }
    }

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(page.replace("${UNITS_CARDS}", &replacement))
}

#[get("/units/{index}")]
async fn unit_page(index: Path<usize>, state: Data<Arc<AppState>>) -> impl Responder {
    let mut page = String::from(include_str!("../website/unit.html"));
    let unit = state.units().get(index.into_inner()).unwrap();
    let mut reviews = String::new();

    for review in unit.reviews() {
        reviews += &review.html();
    }

    // page = page.replace();
    page = page.replace("images/", "/images/");
    page = page.replace("${IMG}", &format!("/images/{}", unit.image()));
    page = page.replace("${IMG_NAME}", unit.image().split('.').next().unwrap());
    page = page.replace("${NAME}", unit.name());
    page = page.replace("${RATING}", &format!("{:0.2}", unit.rating().to_string()));
    page = page.replace("${PRICE}", &format!("{:0.2}", unit.price()));
    page = page.replace("${DESC}", unit.desc());
    page = page.replace("${REVIEWS}", &reviews);

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(page)
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
            .service(unit_page)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
