use actix_web::{middleware::from_fn, web};

use super::{handlers, middlewares::role_middleware};


pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(handlers::donation_handler::get_beneficiaries)
        .service(
        web::scope("/donation")
            .service(handlers::donation_handler::index) // No protegido por roles
            .service(
                web::scope("")
                    .wrap(from_fn(role_middleware::check_donator)) // Aplica solo a create
                    .service(handlers::donation_handler::create),
            )
    );
}