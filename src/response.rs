use serde::{ Serialize, Deserialize };
#[allow(unused_imports)]
use crate::record::Record;
#[allow(unused_imports)]
use crate::sheetcollection::SheetCollection;


//#[derive(Serialize, Deserialize)]
//pub struct RegisterResponse {
//    pub user_id: i64;
//    pub username: String;
//}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub user_id: i64,
    pub username: String,
    pub session_token: String,
    pub bootstrap: BootstrapResponse,
}

#[derive(Serialize, Deserialize)]
pub struct BootstrapResponse {
    pub collections: Vec<SheetCollection>,
}

#[derive(Serialize, Deserialize)]
pub struct GetRecordsResponse {
    pub records: Vec<Record>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRecordResponse {
    pub id: i64,
}
