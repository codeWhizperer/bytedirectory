use actix_web::{
    get, post,
    web::{self}, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use starknet_core::utils::get_selector_from_name;
use uuid::Uuid;
use diesel::prelude::*;
use crate::{model::*, schema::selectors::{self}};
use diesel::Connection;
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


#[get("/selectors")]
async fn get_all_selectors() -> impl Responder {
    let mut connection = db_connect(); // Mutable connection

    match get_selectors(&mut connection) {
        Ok(data) => HttpResponse::Ok().json(SelectorsResponse {
            status: "success".to_string(),
            data
        }),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching selectors"),
    }
}


#[post("/submit")]
async fn submit(req_body: web::Json<FunctionNameRequest>) -> impl Responder {
    let mut connection = db_connect();
    let func_name = req_body.function_name.clone();

    // Check if the function name already exists in the database
    match selectors::table
        .filter(selectors::function_name.eq(&func_name))
        .first::<CreateSelector>(&mut connection)
    {
        Ok(_) => {
            // Function name already exists, return an error
            return HttpResponse::BadRequest().body("Error: Function name already exists");
        }
        Err(diesel::result::Error::NotFound) => {
            // Function name does not exist, proceed to insert
            match get_selector_from_name(&func_name) {
                Ok(felt) => {
                    let felt_hex = format!("{:#x}", felt);
                    let first_4_bytes = &felt_hex[0..10];
                    let new_data = CreateSelector {
                        id: Uuid::new_v4(),
                        function_name: func_name.clone(),
                        felt_selector: felt_hex.clone(),
                        selector: first_4_bytes.to_string(),
                    };

                    // Insert the new data into the database
                    diesel::insert_into(selectors::table)
                        .values(&new_data)
                        .execute(&mut connection)
                        .expect("Error inserting new selector");

                    HttpResponse::Ok().json(new_data)
                }
                Err(e) => HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
            }
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Database error: {:?}", e))
        }
    }
}


#[get("/selectors/{function_name}")]
async fn get_selector_by_name(func_name: web::Path<String>) -> impl Responder {
    let mut connection = db_connect();
    let query = func_name.into_inner();
    match selectors::table
        .filter(selectors::function_name.eq(&query))
        .first::<CreateSelector>(&mut connection)
    {
        Ok(result) => {
            // Use the struct directly instead of destructuring private fields
            let response = SelectorResponse {
                id: result.id,  // Accessing fields directly if they are public
                function_name: result.function_name,
                felt_selector: result.felt_selector,
                selector: result.selector,
            };
            HttpResponse::Ok().json(response)
        }
        Err(diesel::result::Error::NotFound) => {
            // If not found, return a 404 error
            HttpResponse::NotFound().body("Error: Function name not found")
        }
        Err(e) => {
            // Other errors (such as connection issues)
            HttpResponse::InternalServerError().body(format!("Database error: {:?}", e))
        }
    }
}


pub fn get_selectors(conn: &mut PgConnection) -> Result<Vec<SelectorResponse>, diesel::result::Error>{
  let results =  selectors::table.load::<CreateSelector>(conn)?;

  let response: Vec<SelectorResponse> = results
  .into_iter()
  .map(|result| SelectorResponse {
      id: result.id,
      function_name: result.function_name,
      felt_selector: result.felt_selector,
      selector: result.selector,
  })
  .collect();


    Ok(response)
}
