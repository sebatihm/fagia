use actix_web::{middleware::from_fn, web};

use super::{handlers, middlewares::role_middleware};


pub fn config(config: &mut web::ServiceConfig){
    config.service(web::scope("/aliments")
        .wrap(from_fn(role_middleware::check_donator))
        .service(handlers::aliment_handler::index)
        .service(handlers::aliment_handler::show)        
        .service(handlers::aliment_handler::create)
        .service(handlers::aliment_handler::destroy)

    );
}