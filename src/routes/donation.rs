use actix_web::{middleware::from_fn, web};

use super::{handlers, middlewares::role_middleware};


pub fn config(config: &mut web::ServiceConfig){
    config.service(web::scope("/donation")
        .wrap(from_fn(role_middleware::check_donator))
        // .service(super::handlers::aliment_handler::index)
        .service(super::handlers::donation_handler::create)
        // .service(super::handlers::aliment_handler::destroy)

    );
}