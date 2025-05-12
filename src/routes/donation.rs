use actix_web::{middleware::from_fn, web};

use super::{handlers, middlewares::role_middleware};


pub fn config(config: &mut web::ServiceConfig){
    config.service(web::scope("/donation")
        .service(handlers::donation_handler::index)
        .service(web::scope("")
            .wrap(from_fn(role_middleware::check_beneficiary))
            .service(handlers::donation_handler::filter)
            .service(handlers::donation_handler::donator_of_donation)
        )
        .service(web::scope("")
            .wrap(from_fn(role_middleware::check_donator))
            .service(handlers::donation_handler::create)
        )
        .service(handlers::donation_handler::get_beneficiaries)
    );
}