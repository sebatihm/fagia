use actix_web::{middleware::from_fn, web};

use super::{handlers, middlewares::role_middleware};


pub fn config(config: &mut web::ServiceConfig){
    config.service(web::scope("/donation")
        .service(handlers::donation_handler::index)
        .service(web::scope("")
            .wrap(from_fn(role_middleware::check_donator))
            .service(handlers::donation_handler::create)
        )
    );
}