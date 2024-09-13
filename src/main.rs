mod api;


use actix_web::{
    get, post,
    web::{self, service},
    App, HttpResponse, HttpServer, Responder,
};



use api::{submit, hello,echo};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(submit).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
