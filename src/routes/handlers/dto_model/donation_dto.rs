use entity::aliments;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct DonationModel{
    pub date: chrono::NaiveDate,
    pub id_beneficiary: i32,
    pub aliments: Vec<i32>
}

#[derive(Serialize,Deserialize)]
pub struct DTODonation{
    pub id: i32,
    pub date: chrono::NaiveDate,
    pub id_beneficiary: i32,
    pub id_donator: i32,
    pub aliments: Vec<aliments::Model>
}