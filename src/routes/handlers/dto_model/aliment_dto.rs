use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Aliment{
    pub name: String,
    pub r_type: String,
    pub description: String,
    pub lots: i32,
    pub caducity_date: chrono::NaiveDate,
}