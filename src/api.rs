use actix_web::{
    get, post,
    web::{self, service},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use starknet_core::types::Felt;
use starknet_core::utils::get_selector_from_name;
use uuid::Uuid;
use std::env;
use diesel::prelude::*;
use crate::{model::*, schema::selectors::{self, selector}};
use diesel::{Connection, prelude::*};
use diesel::pg::PgConnection;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionSelector {
    pub id: Uuid,
    pub function_name: String,
    pub felt_selector: String,
    pub selector:String
}

fn db_connect() -> PgConnection {


    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("Database Must Be Set");

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", &db_url))

}


#[derive(Deserialize)]
pub struct FunctionNameRequest {
    pub function_name: String,
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// setting up a database

#[post("/submit")]
async fn submit(req_body: web::Json<FunctionNameRequest>) -> impl Responder {
   use crate::schema::selectors::dsl::*;
   let mut connection = db_connect();
    let func_name = req_body.function_name.clone();
    match get_selector_from_name(&func_name) {
        Ok(felt) => {
            let felt_hex = format!("{:#x}", felt);
            let first_4_bytes = &felt_hex[0..10];
            let new_data = CreateSelector {
                id: Uuid::new_v4(),
                function_name:func_name,
                felt_selector: felt_hex.clone(),
                selector:first_4_bytes.to_string()
            };
            diesel::insert_into(selectors)
        .values(&new_data)
        .execute(&mut connection)
        .expect("Error inserting new product");

            HttpResponse::Ok().json(new_data)
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}