mod api;
mod model;
mod schema;

use actix_web::{
    App, HttpServer,
};



use api::{submit, get_all_selectors,get_selector_by_name};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_all_selectors).service(submit).service(get_selector_by_name))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

