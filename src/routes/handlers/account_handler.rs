use actix_web::HttpResponse;
use actix_web::{ post, web, Responder};
use sea_orm::{ActiveValue::Set, Condition, EntityTrait, QueryFilter};
use sha256::digest;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;


use crate::routes::handlers::dto_model::account_dto::{LoginModel, Message, RegisterBeneficiaryModel, RegisterDonatorModel, Token};
use crate::utils::app_state::AppState;
use crate::utils::jwt::encode_jwt;



async fn register_credentials(app_state: &web::Data<AppState>, credentials: LoginModel) -> Result<String,String>{
    let user = entity::credentials::Entity::find()
        .filter(
            Condition::all()
                .add(entity::credentials::Column::Email.eq(&credentials.email))
    ).one(&app_state.db).await.unwrap();

    match user {
        Some(_data) => {
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
    };

    match register_credentials(&app_state, cred).await{
        Ok(_) =>{
            return HttpResponse::BadRequest().json("Email Already Taken");
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
                organization_name: Set(register_json.organization_name.clone()), 
                credentials_id: Set(credentials_model.id), 
                ..Default::default() 
                
            }.insert(&app_state.db).await.unwrap();
        
            
            return HttpResponse::Created()
                .json(donator_model);
        },
    };

    
}


#[post("/register-beneficiary")]
pub async fn register_beneficiary(app_state: web::Data<AppState>,register_json: web::Json<RegisterBeneficiaryModel>) -> impl Responder{


    let creds = LoginModel{ 
        email: register_json.email.clone(), 
        password: register_json.password.clone(), 
    };

    match register_credentials(&app_state, creds).await {
        Ok(_) =>{
            return HttpResponse::BadRequest().json("Email Already Taken");
        },
        Err(_) => {
            let nif_taken = entity::beneficiary::Entity::find().filter(
                Condition::all()
                    .add(entity::beneficiary::Column::Nif.eq(register_json.nif.clone()))
             ).one(&app_state.db).await.unwrap();
        
            if nif_taken != None {
                return HttpResponse::BadRequest().json("NIF Already Taken");
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
                nif: Set(register_json.nif.clone()),         
                website: Set(register_json.website.clone()),
                foundation_date: Set(register_json.foundation_date.clone()), 
                credentials_id: Set(credentials_model.id.clone()),
                ..Default::default()
            }.insert(&app_state.db).await.unwrap();
        
        
            return HttpResponse::Created()
                .json(beneficiary_model);
            },
    }
        
    

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
            let token = encode_jwt(data.email, data.id, data.r_type).unwrap();

            return HttpResponse::Ok()
                .json(Token{ token: token});
        },

        None => {
            return HttpResponse::BadRequest().json(Message{message: "Something went wrong".to_string()});
        }
    };
    
}