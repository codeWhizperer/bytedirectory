use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::selectors;
use diesel::prelude::*;


#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "selectors"]
pub struct CreateSelector{
    pub id: Uuid,
    pub function_name:String,
    pub felt_selector:String,
    pub selector: String
}

#[derive(Serialize)]
pub struct SelectorResponse{
    pub id: Uuid,
    pub function_name:String,
    pub felt_selector:String,
    pub selector: String
}

#[derive(Serialize)]
pub struct SelectorsResponse{
    pub status:String,
    pub data: Vec<SelectorResponse>
}
