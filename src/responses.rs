use serde::{ Serialize, Deserialize };
use crate::record::Record;
use crate::sheetcollection::SheetCollection;

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
