use actix_web::web;

use super::handlers;



pub fn config(config: &mut web::ServiceConfig ){
    config.service(web::scope("/test")
        .service(handlers::test_handlers::init)

    );
}