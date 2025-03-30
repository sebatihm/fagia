use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use super::constants;

#[derive(Serialize, Deserialize)]
pub struct Claims{
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: i32,
    pub role: String
}

pub fn encode_jwt(email: String, id:i32, rol: entity::sea_orm_active_enums::RType) -> Result<String, jsonwebtoken::errors::Error>{
    let now = Utc::now();
    let expire = Duration::hours(2);
    let role: String;

    if rol == entity::sea_orm_active_enums::RType::Beneficiary{
        role = String::from("Beneficiary");
    }else{
        role = String::from("Donator");
    }
    let claims: Claims = Claims{
        exp: (now+expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id,
        role
    };

    let secret = (*constants::SECRET).clone();

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))

    
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>,jsonwebtoken::errors::Error> {
    let secret = (*constants::SECRET).clone();
    let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
        &jwt, 
        &DecodingKey::from_secret(secret.as_ref()), 
        &Validation::default()
    );

    claim_data
}