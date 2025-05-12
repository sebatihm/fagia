use actix_web::{delete, get, web, HttpMessage, HttpRequest, HttpResponse};
use entity::{credentials};
use sea_orm::{ConnectionTrait, DatabaseBackend, EntityTrait, ModelTrait, Statement};

use crate::{routes::handlers::dto_model::{account_dto::Message, profile_dto::{BeneficiaryData, DonatorData}}, utils::{app_state::AppState, jwt::Claims}};

async fn get_credentials(id: i32 ,app_state: &web::Data<AppState>)-> Option<credentials::Model>{
    let credentials = credentials::Entity::find_by_id(id).one(&app_state.db).await.unwrap();
    return credentials;

}

#[get("")]
pub async fn info_account(req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse{
    let account =get_credentials(req.extensions().get::<Claims>().unwrap().id, &app_state).await;

    if account == None{
        return HttpResponse::Unauthorized().json(Message{message: format!("Something went wrong")});
    }

    let beneficiary = account.clone().unwrap().find_related(entity::beneficiary::Entity).one(&app_state.db).await.unwrap();
    let donator = account.clone().unwrap().find_related(entity::donator::Entity).one(&app_state.db).await.unwrap();


    if beneficiary != None{
        let beneficiary = beneficiary.unwrap();
        return HttpResponse::Ok()
            .json(BeneficiaryData{ creds: account.unwrap(), data: beneficiary });
    } else if donator != None{
        let donator = donator.unwrap();
        return HttpResponse::Ok()
            .json(DonatorData{ creds: account.unwrap(), data: donator });
    }else {
        return HttpResponse::BadRequest().json(Message{message: format!("Something went wrong")});
    }

}

#[delete("")]
pub async fn delete_account(req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse {
    let db = &app_state.db;

    // 🔻 Desactivar constraints
    let _ = db.execute(Statement::from_string(DatabaseBackend::MySql, "SET FOREIGN_KEY_CHECKS = 0".to_owned())).await;

    let account = entity::credentials::Entity::find_by_id(req.extensions().get::<Claims>().unwrap().id)
        .one(db)
        .await
        .unwrap()
        .unwrap();

    
    let result = account.delete(db).await;

    // 🔺 Reactivar constraints
    let _ = db.execute(Statement::from_string(DatabaseBackend::MySql, "SET FOREIGN_KEY_CHECKS = 1".to_owned())).await;

    match result {
        Ok(_) => HttpResponse::NoContent().json(Message {
            message: "Account Deleted successfully".to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(Message {
            message: format!("Something went wrong {err}"),
        }),
    }
}