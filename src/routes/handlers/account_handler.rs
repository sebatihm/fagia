use std::collections::btree_map::Entry;

use actix_web::error::{ErrorBadGateway, HttpError};
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::{get, post, web, Responder};
use entity::credentials;
use sea_orm::{ActiveValue::Set, Condition, ConnectionTrait, EntityTrait, QueryFilter, Statement};
use serde::{Deserialize, Serialize};
use sha256::digest;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use crate::utils::app_state::{self, AppState};
use crate::utils::jwt::{self, encode_jwt};


#[derive(Serialize, Deserialize)]
struct RegisterDonatorModel{
    name: String,
    lastname_f: String,
    lastname_m: String,
    phone: String,
    organization_name: String,
    email: String,
    password: String,
    r_type: entity::sea_orm_active_enums::RType
}


async fn register_credentials(app_state: &web::Data<AppState>, credentials: LoginModel) -> Result<String,String>{
    let user = entity::credentials::Entity::find()
        .filter(
            Condition::all()
                .add(entity::credentials::Column::Email.eq(&credentials.email))
                .add(entity::credentials::Column::Password.eq (digest(&credentials.password)))
    ).one(&app_state.db).await.unwrap();

    match user {
        Some(data) => {
            Ok(String::from("INVALID"))
        },
        None => {
            Err(String::from("VALID"))
        }
    }
}


#[post("/register-donator")]
pub async fn register_donator(app_state: web::Data<AppState>, register_json: web::Json<RegisterDonatorModel>) -> HttpResponse{

    let cred = LoginModel{ 
        email: register_json.email.clone(), 
        password: register_json.password.clone(), 
        r_type: register_json.r_type.clone() 
    };

    match register_credentials(&app_state, cred).await{
        Ok(_) =>{
            return HttpResponse::BadRequest().body("Email Already Taken");
        },
        Err(_) => {
            
            let credentials_model = entity::credentials::ActiveModel{ 
                email: Set(register_json.email.clone()), 
                password: Set(digest(register_json.password.clone())), 
                r_type: Set(register_json.r_type.clone()),
                ..Default::default() 
                
            }.insert(&app_state.db).await.unwrap();
        
            
            let donator_model = entity::donator::ActiveModel{ 
                name: Set(register_json.name.clone()) , 
                lastname_f: Set(register_json.lastname_f.clone()), 
                lastname_m: Set(register_json.lastname_m.clone()), 
                phone: Set(register_json.phone.clone()), 
                organization_name: Set(register_json.phone.clone()), 
                credentials_id: Set(credentials_model.id), 
                ..Default::default() 
                
            }.insert(&app_state.db).await.unwrap();
        
            
            return HttpResponse::Ok()
                .status(StatusCode::from_u16(201).unwrap())
                .json(donator_model);
        },
    };

    
}



#[derive(Serialize, Deserialize)]
struct RegisterBeneficiaryModel{
    representant_name: String,
    representant_lastname_f: String,
    representant_lastname_m: String,
    phone: String,
    legal_name: String,
    foundation_date: chrono::NaiveDate,
    NIF: String,
    website: String,
    email: String,
    password: String,
    r_type: entity::sea_orm_active_enums::RType
}

#[post("/register-beneficiary")]
pub async fn register_beneficiary(app_state: web::Data<AppState>,register_json: web::Json<RegisterBeneficiaryModel>) -> impl Responder{
    // if entity::beneficiary::Entity::find()
    //     .filter(
    //         Condition::all()
    //             .add(entity::beneficiary::Column::Nif.eq(register_json.NIF.clone()))
    //     ).one(&app_state.db).await{

    let creds = LoginModel{ 
        email: register_json.email.clone(), 
        password: register_json.password.clone(), 
        r_type: register_json.r_type.clone() 
    };

    match register_credentials(&app_state, creds).await {
        Ok(_) =>{
            return HttpResponse::BadRequest().body("Email Already Taken");
        },
        Err(_) => {
            let nif_taken = entity::beneficiary::Entity::find().filter(
                Condition::all()
                    .add(entity::beneficiary::Column::Nif.eq(register_json.NIF.clone()))
             ).one(&app_state.db).await.unwrap();
        
            if nif_taken != None {
                return HttpResponse::BadRequest().body("NIF Already Taken");
            }



            let credentials_model = entity::credentials::ActiveModel{ 
                email: Set(register_json.email.clone()), 
                password: Set(digest(register_json.password.clone())), 
                r_type: Set(register_json.r_type.clone()),
                ..Default::default()
            }.insert(&app_state.db).await.unwrap();
        
            let beneficiary_model = entity::beneficiary::ActiveModel{ 
                representant_name: Set(register_json.representant_name.clone()), 
                representant_lastname_f: Set(register_json.representant_lastname_f.clone()), 
                representant_lastname_m: Set(register_json.representant_lastname_m.clone()), 
                phone: Set(register_json.phone.clone()), 
                legal_name: Set(register_json.legal_name.clone()), 
                nif: Set(register_json.NIF.clone()),         
                website: Set(register_json.website.clone()),
                foundation_date: Set(register_json.foundation_date.clone()), 
                credentials_id: Set(credentials_model.id.clone()),
                ..Default::default()
            }.insert(&app_state.db).await.unwrap();
        
        
            return HttpResponse::Ok()
                .status(StatusCode::from_u16(201).unwrap())
                .json(beneficiary_model);
            },
    }
        
    

}

#[derive(Serialize,Deserialize)]
pub struct LoginModel{
        email: String,
        password: String,
        r_type: entity::sea_orm_active_enums::RType
}

#[post("/login")]
pub async fn login(app_state: web::Data<AppState>, login_json: web::Json<LoginModel>) -> HttpResponse{
    
    let user = entity::credentials::Entity::find()
        .filter(
            Condition::all()
                .add(entity::credentials::Column::Email.eq(&login_json.email))
                .add(entity::credentials::Column::Password.eq (digest(&login_json.password)))
        ).one(&app_state.db).await.unwrap();

    match user {

        Some(data) => {
            let token = encode_jwt(data.email, data.id).unwrap();

            return HttpResponse::Ok()
                .status(StatusCode::from_u16(201).unwrap())
                .body(format!("token: {{ {token} }}"));
        },

        None => {
            return HttpResponse::BadRequest().body("The credentials dont match");
        }
    };
    
}