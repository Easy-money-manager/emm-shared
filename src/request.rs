use serde::{ Serialize, Deserialize };
#[allow(unused_imports)]
use crate::record::Record;
#[allow(unused_imports)]
use crate::sheetcollection::SheetCollection;


#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
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

#[derive(Serialize, Deserialize)]
pub struct ImportSheetRequest {
    pub records: Vec<ParsedImportRecord>,
}
