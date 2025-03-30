use actix_web::{http::StatusCode, post, web, HttpMessage, HttpRequest, HttpResponse};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use sea_orm::ActiveModelTrait;
use crate::utils::{app_state::AppState, jwt::Claims};

#[derive(Serialize,Deserialize)]
struct Aliment{
    pub name: String,
    pub r_type: String,
    pub description: String,
    pub lots: i32,
    pub caducity_date: chrono::NaiveDate,
}

#[post("/create")]
pub async fn create(req: HttpRequest, app_state: web::Data<AppState>, aliment_json: web::Json<Aliment>) -> HttpResponse{


    let aliment_model = entity::aliments::ActiveModel{ 
        name: Set(aliment_json.name.clone()), 
        r_type: Set(aliment_json.r_type.clone()), 
        description: Set(aliment_json.description.clone()), 
        lots: Set(aliment_json.lots), 
        caducity_date: Set(aliment_json.caducity_date), 
        id_donator: Set(req.extensions().get::<Claims>().unwrap().id.clone()),
        ..Default::default()
    }.insert(&app_state.db).await.unwrap();

    return HttpResponse::Ok()
        .status(StatusCode::from_u16(201).unwrap())
        .json(aliment_model);
        


    
}