mod api;
mod model;
mod schema;

use std::any::Any;

use actix_cors::Cors;
use actix_web::{
    http::{
        self,
        header::{self, CONTENT_TYPE},
        Method,
    },
    web::head,
    App, HttpServer,
};

use api::{get_all_selectors, get_selector_by_name, submit};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("https://byte-frontend-gpre.vercel.app")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(get_all_selectors)
            .service(submit)
            .service(get_selector_by_name)
    })
    .bind(())?
    .run()
    .await
}
