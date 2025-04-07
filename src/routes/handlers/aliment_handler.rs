use std::u32;
use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse};
use entity::{aliments, donator};
use sea_orm::{ActiveValue::Set, Condition, DeleteResult, EntityTrait, QueryFilter};
use sea_orm::{ActiveModelTrait,ColumnTrait, ModelTrait};
use crate::utils::{app_state::AppState, jwt::Claims};
use super::dto_model::aliment_dto::Aliment;



async fn get_donator(id: i32,app_state: &web::Data<AppState>) -> Option<donator::Model>{

    let donator = entity::donator::Entity::find()
        .filter(
            Condition::all()
                .add(entity::donator::Column::CredentialsId.eq(id))
    ).one(&app_state.db).await.unwrap();
    donator
}

#[get("")]
pub async fn index(req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse{
    let donator: Option<donator::Model> = get_donator(req.extensions().get::<Claims>().unwrap().id, &app_state).await;
    let results = donator.unwrap().find_related(aliments::Entity).all(&app_state.db).await.unwrap();

    return HttpResponse::Ok().json(results);
}

#[post("")]
pub async fn create(req: HttpRequest, app_state: web::Data<AppState>, aliment_json: web::Json<Aliment>) -> HttpResponse{
    let donator: Option<donator::Model> = get_donator(req.extensions().get::<Claims>().unwrap().id, &app_state).await;


    let aliment_model = aliments::ActiveModel{ 
        name: Set(aliment_json.name.clone()), 
        r_type: Set(aliment_json.r_type.clone()), 
        description: Set(aliment_json.description.clone()), 
        lots: Set(aliment_json.lots), 
        caducity_date: Set(aliment_json.caducity_date), 
        id_donator: Set(donator.unwrap().id),
        ..Default::default()
    }.insert(&app_state.db).await.unwrap();

    return HttpResponse::Created().json(aliment_model);
    
}

#[post("/{id}")]
pub async fn show(req: HttpRequest, app_state: web::Data<AppState>, id: web::Path<u32>) -> HttpResponse{
    let donator = get_donator(req.extensions().get::<Claims>().unwrap().id, &app_state).await;
    let aliment = aliments::Entity::find()
        .filter(
            Condition::all()
                .add(aliments::Column::Id.eq(id.into_inner()))
    ).one(&app_state.db).await.unwrap();

    match aliment{
        Some(data) =>{
            if data.id_donator == donator.unwrap().id{
                return HttpResponse::Ok()
                    .json(data);
            }else{
                return HttpResponse::Unauthorized().body("The aliment doesnt belong to the authentificated user");
            }
        },
        None => {
            return HttpResponse::BadRequest().body("The aliment doesnt exist");
        },
    }
    


}

#[delete("/{id}")]
pub async fn destroy(req: HttpRequest, app_state: web::Data<AppState>, id: web::Path<u32>) -> HttpResponse{
    let donator = get_donator(req.extensions().get::<Claims>().unwrap().id, &app_state).await;

    let aliment = aliments::Entity::find()
        .filter(
            Condition::all()
                .add(aliments::Column::Id.eq(id.into_inner()))
    ).one(&app_state.db).await.unwrap();
    

    match aliment {
        Some(data) => {

            if data.id_donator == donator.unwrap().id{
                let _res: DeleteResult = data.delete(&app_state.db).await.unwrap();

                return HttpResponse::Ok()
                    .body("The operation was succesfull");
            }else{
                return HttpResponse::Unauthorized().body("You are not the owner of this aliment");
            };
           
        },
        None => {
            return HttpResponse::NotFound().body("The resource doesnt exist");
        },
    }
}