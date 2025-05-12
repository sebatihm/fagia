use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize)]
pub struct BeneficiaryData{
    pub creds: entity::credentials::Model,
    pub data: entity::beneficiary::Model
}

#[derive(Serialize,Deserialize)]
pub struct DonatorData{
    pub creds: entity::credentials::Model,
    pub data: entity::donator::Model
}

