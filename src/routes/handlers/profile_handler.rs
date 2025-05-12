use actix_web::{delete, web, HttpMessage, HttpRequest, HttpResponse};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait, DatabaseBackend, EntityTrait, ModelTrait, Statement};

use crate::{routes::handlers::dto_model::account_dto::Message, utils::{app_state::AppState, jwt::Claims}};


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