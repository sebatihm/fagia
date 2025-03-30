use actix_web::dev::ServiceResponse;
use actix_web::error::{ErrorBadGateway, ErrorUnauthorized};
use actix_web::http::header::AUTHORIZATION;
use actix_web::{Error, HttpMessage, HttpRequest};
use actix_web::{body::MessageBody, dev::ServiceRequest};
use actix_web::middleware::Next;

use crate::utils::jwt::decode_jwt;

//Middleware function to authorize
pub async fn check_auth_middleware( req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error>{
    let auth = req.headers().get(AUTHORIZATION);
    
    if auth.is_none(){
        return Err(ErrorBadGateway("Unauthorized"));
    }

    let token = auth.unwrap().to_str().unwrap().replace("Bearer ", "").to_owned();
    let claim = decode_jwt(token).unwrap().claims;
    req.extensions_mut().insert(claim);
    
    
    next.call(req).await
    .map_err(|err |ErrorUnauthorized(err) )
}
