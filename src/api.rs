use actix_web::{
    get, post,
    web::{self, service},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use starknet_core::types::Felt;
use starknet_core::utils::get_selector_from_name;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionSelector {
    pub id: Uuid,
    pub function_name: String,
    pub felt_selector: Felt,
    pub selector:String
}



#[derive(Deserialize)]
pub struct FunctionNameRequest {
    pub function_name: String,
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[post("/submit")]
async fn submit(req_body: web::Json<FunctionNameRequest>) -> impl Responder {
    let function_name = req_body.function_name.clone();
    match get_selector_from_name(&function_name) {
        Ok(felt_selector) => {
            let felt_hex = format!("{:#x}", felt_selector);
            let first_4_bytes = &felt_hex[0..10];
            let new_data = FunctionSelector {
                id: Uuid::new_v4(),
                function_name,
                felt_selector: felt_selector,
                selector:first_4_bytes.to_string()
            };
            HttpResponse::Ok().json(new_data)
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {:?}", e)),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}