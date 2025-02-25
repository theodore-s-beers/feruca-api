#![warn(clippy::pedantic, clippy::nursery)]

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use feruca::{Collator, Locale, Tailoring};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SortReq {
    items: Vec<String>,
    tailoring: Option<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("https://www.theobeers.com")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type", "Accept"])
                    .max_age(3600),
            )
            .service(hello)
            .service(sorter)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}

#[post("/")]
async fn sorter(payload: web::Json<SortReq>) -> impl Responder {
    let tailoring: Tailoring = payload
        .tailoring
        .as_ref()
        .map_or_else(Tailoring::default, |t| match t.as_str() {
            "ArabicScript" => Tailoring::Cldr(Locale::ArabicScript),
            "ArabicInterleaved" => Tailoring::Cldr(Locale::ArabicInterleaved),
            _ => Tailoring::default(),
        });

    let mut collator = Collator::new(tailoring, true, true);

    let mut list = payload.items.clone();
    list.sort_unstable_by(|a, b| collator.collate(a, b));

    HttpResponse::Ok().json(list)
}
