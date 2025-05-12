use actix_web::{delete, web, HttpMessage, HttpRequest, HttpResponse};
use sea_orm::{EntityTrait, ModelTrait};

use crate::{routes::handlers::dto_model::account_dto::Message, utils::{app_state::AppState, jwt::Claims}};


#[delete("")]
pub async fn delete_account(req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse {
    let account = entity::credentials::Entity::find_by_id(req.extensions().get::<Claims>().unwrap().id).one(&app_state.db).await.unwrap().unwrap();
    let result= account.delete(&app_state.db).await;

    match result {
        Ok(_) => return HttpResponse::Ok().json(Message{message: format!("Borrado correctamente") }),
        Err(err) => return HttpResponse::InternalServerError().json(Message{ message: format!("Something went wrong {err}") }),
    }
    

}