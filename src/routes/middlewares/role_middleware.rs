use actix_web::dev::ServiceResponse;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{Error, HttpMessage};
use actix_web::{body::MessageBody, dev::ServiceRequest};
use actix_web::middleware::Next;

use crate::utils::jwt::Claims;

pub async fn check_beneficiary( req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error>{
    let auth = req.headers().get(AUTHORIZATION);
    
    if auth.is_none(){
        return Err(ErrorUnauthorized("Unauthorized - The JWT wasnt specified"));
    }

    if req.extensions().get::<Claims>().unwrap().role != String::from("Beneficiary"){
        return Err(ErrorUnauthorized("Unauthorized - The user is not a Beneficiary"));
    }
    
    next.call(req).await
    .map_err(|err |ErrorUnauthorized(err) )
}



pub async fn check_donator( req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error>{
    let auth = req.headers().get(AUTHORIZATION);
    
    if auth.is_none(){
        return Err(ErrorUnauthorized("Unauthorized - The JWT wasnt specified"));
    }

    if req.extensions().get::<Claims>().unwrap().role != String::from("Donator"){
        return Err(ErrorUnauthorized("Unauthorized - The user is not a donator"));
    }
    
    next.call(req).await
    .map_err(|err |ErrorUnauthorized(err) )
}