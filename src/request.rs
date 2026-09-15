use serde::{ Serialize, Deserialize };
use crate::record::Record;
use crate::sheetcollection::SheetCollection;


#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String;
    pub password: String;
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String;
    pub password: String;
}

#[derive(Serialize, Deserialize)]
pub struct CreateRecordRequest {
    pub description: String,
    pub date: chrono::NaiveDate,
    pub value: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateRecordRequest {
    pub description: String,
    pub date: chrono::NaiveDate,
    pub value: i64,
}


//#[derive(Serialize)]
//struct GetRecordsRequest {
//}

//#[derive(Serialize, Deserialize)]
//pub struct UpdateRecordResponse {
//}

//#[derive(Serialize)]
//pub struct RemoveRecordRequest {
//}

//#[derive(Serialize, Deserialize)]
//pub struct RemoveRecordResponse {
//}

