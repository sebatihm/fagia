use actix_web::web;

use super::handlers;



pub fn config(config: &mut web::ServiceConfig ){
    config.service(web::scope("/account")
        .service(handlers::profile_handler::delete_account)

    );
}