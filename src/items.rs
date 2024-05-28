use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Items {
    pub origin: Position,
    pub radius: i32,
    pub page_size: i32,
    pub page_number: i32,
    pub discover: bool,
    pub favorites_only: bool,
    pub item_categories: Vec<String>,
    pub diet_categories: Vec<String>,
    pub pickup_earliest: Vec<String>,
    pub pickup_latest: Vec<String>,
    pub search_phrase: String,
    pub with_stock_only: bool,
    pub hidden_only: bool,
    pub we_care_only: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub latitude: f32,
    pub longitude: f32,
}