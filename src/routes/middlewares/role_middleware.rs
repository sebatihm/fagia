use actix_web::dev::ServiceResponse;
use actix_web::error::{ErrorUnauthorized, InternalError};
use actix_web::http::header::{self, AUTHORIZATION};
use actix_web::{Error, HttpMessage, HttpResponse};
use actix_web::{body::MessageBody, dev::ServiceRequest};
use actix_web::middleware::Next;

use crate::utils::jwt::Claims;

pub async fn check_beneficiary( req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error>{
    let auth = req.headers().get(AUTHORIZATION);
    
    if auth.is_none(){
        let response = HttpResponse::Unauthorized()
            .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
            .json("The User Must specify the JWT");

        return Err(InternalError::from_response("Unauthorized", response).into());
    }

    if req.extensions().get::<Claims>().unwrap().role != String::from("Beneficiary"){
        let response = HttpResponse::Unauthorized()
        .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .json("The User is not a Beneficiary");

        return Err(InternalError::from_response("Unauthorized", response).into());
    }
    
    next.call(req).await
    .map_err(|err |ErrorUnauthorized(err) )
}



pub async fn check_donator( req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error>{
    let auth = req.headers().get(AUTHORIZATION);
    
    if auth.is_none(){
        let response = HttpResponse::Unauthorized()
            .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
            .json("The User Must specify the JWT");

        return Err(InternalError::from_response("Unauthorized", response).into());
    }

    if req.extensions().get::<Claims>().unwrap().role != String::from("Donator"){

        let response = HttpResponse::Unauthorized()
            .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
            .json("The User is not a Donator");

        return Err(InternalError::from_response("Unauthorized", response).into());
    }
    
    next.call(req).await
    .map_err(|err |ErrorUnauthorized(err) )
}