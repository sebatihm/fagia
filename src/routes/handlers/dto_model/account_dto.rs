use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct LoginModel{
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterBeneficiaryModel{
    pub representant_name: String,
    pub representant_lastname_f: String,
    pub representant_lastname_m: String,
    pub phone: String,
    pub legal_name: String,
    pub foundation_date: chrono::NaiveDate,
    pub nif: String,
    pub website: String,
    pub email: String,
    pub password: String,
    pub r_type: entity::sea_orm_active_enums::RType
}


#[derive(Serialize, Deserialize)]
pub struct RegisterDonatorModel{
    pub name: String,
    pub lastname_f: String,
    pub lastname_m: String,
    pub phone: String,
    pub organization_name: String,
    pub email: String,
    pub password: String,
    pub r_type: entity::sea_orm_active_enums::RType
}