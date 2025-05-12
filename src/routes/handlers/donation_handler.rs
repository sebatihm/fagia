use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse,get};
use entity::{beneficiary, credentials, donation, donator};
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm::{ActiveModelTrait, Condition, ModelTrait, QueryFilter};
use sea_orm::ColumnTrait;

use crate::routes::handlers::dto_model::account_dto::Message;
use crate::utils::{app_state::AppState, jwt::Claims};
use super::dto_model::donation_dto::{DTODonation, DonationModel};

use chrono::{NaiveDate, Duration};



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
        return HttpResponse::BadRequest().json(Message{message: format!("The beneficiary doesnt exists")});
    }


    for i in &donation_handler.aliments{

        let _aliment = entity::aliments::Entity::find_by_id(i.clone()).one(&app_state.db).await.unwrap();

        if _aliment == None{
            return HttpResponse::BadRequest().json(Message{message: format!("The aliment with id: {} does not exist",i)});
        }

        if _aliment.clone().unwrap().id_donator != donator.clone().unwrap().id {
            return HttpResponse::Unauthorized().json(Message{message: format!("The aliment with id: {} has already been donated!",i)});
        }else if _aliment.unwrap().find_related(entity::aliment_per_donation::Entity).one(&app_state.db).await.unwrap() != None{
            return HttpResponse::BadRequest().json(Message{message: format!("The aliment with id: {} has already been donated!",i)});
        }
        
    }

    let donation_record = donation::ActiveModel{
        date: Set(donation_handler.date.clone()),
        id_donator: Set(donator.clone().unwrap().id ),
        id_beneficiary: Set(donation_handler.id_beneficiary.clone()),
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
        id_beneficiary: donation_record.id_beneficiary,
        aliments: donation_record.find_related(entity::aliments::Entity).all(&app_state.db).await.unwrap(),
    };

    return HttpResponse::Created().json(response);

}


#[get("/filter/{days}")]
pub async fn filter(req: HttpRequest ,app_state: web::Data<AppState>,days: web::Path<i64>)-> HttpResponse{
    
    let credentials = get_credentials(req.extensions().get::<Claims>().unwrap().id.clone(), &app_state).await.unwrap();
    
    let today: NaiveDate = chrono::Local::now().naive_local().date();

    let date_filter = today - Duration::days(days.abs());

    let beneficiary = credentials.find_related(beneficiary::Entity).one(&app_state.db).await.unwrap().unwrap();



    let donations = beneficiary.find_related(entity::donation::Entity).filter(
        Condition::all()
            .add(entity::donation::Column::Date.gte(date_filter))
    ).all(&app_state.db).await.unwrap();

    if donations.is_empty(){
        return HttpResponse::Ok().json(Message{message: "There are no donations".to_string()});
    }

    let mut dtodonations : Vec<DTODonation> = vec![];

    for donation_record in donations{
        dtodonations.push(DTODonation{
            id: donation_record.id,
            date: donation_record.date,
            id_donator: donation_record.id_donator,
            id_beneficiary: donation_record.id_beneficiary,
            aliments: donation_record.find_related(entity::aliments::Entity).all(&app_state.db).await.unwrap(),
        });
    }
    return HttpResponse::Ok().json(dtodonations);
    
}


#[get("/{id}/donator")]
pub async fn donator_of_donation(req: HttpRequest ,app_state: web::Data<AppState>,donation: web::Path<i32>)-> HttpResponse{
    let credentials = get_credentials(req.extensions().get::<Claims>().unwrap().id.clone(), &app_state).await.unwrap();
    let beneficiary =  credentials.find_related(beneficiary::Entity).one(&app_state.db).await.unwrap().unwrap();
    let donation = donation::Entity::find_by_id(donation.abs()).one(&app_state.db).await.unwrap();

    if donation == None {
        return HttpResponse::BadRequest().json(Message{message: "The donation doesnt exists".to_string()});
    }else if donation.clone().unwrap().id_beneficiary != beneficiary.id{
        return HttpResponse::Unauthorized().json(Message{message: "The authenticated user did not receive that donation.".to_string()});
    }

    let donator = donation.unwrap().find_related(entity::donator::Entity).one(&app_state.db).await.unwrap().unwrap();

    return HttpResponse::Ok().json(donator);
    
}

#[get("/beneficiaries")]
pub async fn get_beneficiaries(app_state: web::Data<AppState>) -> HttpResponse{
    let beneficiaries = beneficiary::Entity::find().filter(
        Condition::all()
            .add(entity::beneficiary::Column::CredentialsId.ne(0))
    ) .all(&app_state.db).await;

    match beneficiaries {
        Ok(models) => return HttpResponse::Ok().json(models),
        Err(_) => return HttpResponse::InternalServerError().json(Message{message: "Something went wrong".to_string()}),
    }

}