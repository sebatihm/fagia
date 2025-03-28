use actix_web::web;

use super::handlers;


pub fn config(config: &mut web::ServiceConfig ){
    config.service(web::scope("/accounts")
        .service(handlers::account_handler::register_beneficiary)
        .service(handlers::account_handler::register_donator)
        .service(handlers::account_handler::login)
    );
}