use actix_web::{middleware::from_fn, web};

use super::{handlers, middlewares::role_middleware};


pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/beneficiary/donation")
            .service(handlers::donation_handler::index) // No protegido por roles
            .service(
                web::scope("")
                    .wrap(from_fn(role_middleware::check_beneficiary)) // Aplica a filter y donator_of_donation
                    .service(handlers::donation_handler::filter)
                    .service(handlers::donation_handler::donator_of_donation),
            )
    );
}

