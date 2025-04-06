use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse,get};
use entity::{beneficiary, credentials, donation, donator};
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm::{Condition, DeleteResult, QueryFilter,ColumnTrait};
use sea_orm::{ActiveModelTrait, ModelTrait, Related};
use crate::utils::{app_state::AppState, jwt::Claims};

use super::dto_model::donation_dto::{DTODonation, DonationModel};

async fn get_credentials(id: i32 ,app_state: &web::Data<AppState>)-> Option<credentials::Model>{
    let credentials = credentials::Entity::find_by_id(id).one(&app_state.db).await.unwrap();
    return credentials;

}

#[get("")]
pub async fn index(req: HttpRequest ,app_state: web::Data<AppState>)-> HttpResponse{
    let credentials = get_credentials(req.extensions().get::<Claims>().unwrap().id.clone(), &app_state).await.unwrap();
    if req.extensions().get::<Claims>().unwrap().role == "Donator"{
        let donator = credentials.find_related(donator::Entity).one(&app_state.db).await.unwrap().unwrap();
        let donations = donator.find_related(entity::donation::Entity).all(&app_state.db).await.unwrap();

        return HttpResponse::Ok().json(donations);
    }else{
        let beneficiary = credentials.find_related(beneficiary::Entity).one(&app_state.db).await.unwrap().unwrap();
        let donations = beneficiary.find_related(entity::donation::Entity).all(&app_state.db).await.unwrap();
        return HttpResponse::Ok().json(donations);


    }
}


#[post("")]
pub async fn create(req: HttpRequest ,app_state: web::Data<AppState>,donation_handler: web::Json<DonationModel>) -> HttpResponse{
    let credentials = get_credentials(req.extensions().get::<Claims>().unwrap().id.clone(), &app_state).await.unwrap();
    let donator = credentials.find_related(donator::Entity).one(&app_state.db).await.unwrap();
    
    let beneficiary = entity::beneficiary::Entity::find_by_id(donation_handler.id_beneficiary).one(&app_state.db).await.unwrap();
    
    if beneficiary == None{
        return HttpResponse::BadRequest().body("The beneficiary doesnt exists");
    }


    for i in &donation_handler.aliments{

        let _aliment = entity::aliments::Entity::find_by_id(i.clone()).one(&app_state.db).await.unwrap();

        if _aliment == None{
            return HttpResponse::BadRequest().body(format!("The aliment with id: {} does not exist",i));
        }

        if _aliment.clone().unwrap().id_donator != donator.clone().unwrap().id {
            return HttpResponse::Unauthorized().body(format!("The aliment with id: {} does not belong to the autentificated user!",i));
        }else if _aliment.unwrap().find_related(entity::aliment_per_donation::Entity).one(&app_state.db).await.unwrap() != None{
            return HttpResponse::BadRequest().body(format!("The aliment with id: {} has already been donated!",i));
        }
        
    }

    let donation_record = donation::ActiveModel{
        date: Set(donation_handler.date.clone()),
        id_donator: Set(donator.clone().unwrap().id ),
        id_benefactor: Set(donation_handler.id_beneficiary.clone()),
        ..Default::default()
    }.insert(&app_state.db).await.unwrap();

    for i in &donation_handler.aliments{
        let _aliment_per_donation = entity::aliment_per_donation::ActiveModel{
            id_aliments: Set(i.clone()),
            id_donation: Set(donation_record.id.clone()),
        }.insert(&app_state.db).await.unwrap();
    }

    let response = DTODonation{
        id: donation_record.id,
        date: donation_record.date,
        id_donator: donation_record.id_donator,
        id_beneficiary: donation_record.id_benefactor,
        aliments: donation_record.find_related(entity::aliments::Entity).all(&app_state.db).await.unwrap(),
    };

    return HttpResponse::Created().json(response);

}
