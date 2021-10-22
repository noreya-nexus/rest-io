#![allow(dead_code)]

use rocket::response::content::Json;
use rocket::response::Responder;

#[derive(Responder)]
pub enum CResponse {
    #[response(status = 200)]
    Ok(Json<String>),
    #[response(status = 400)]
    BadRequest(Json<String>),
    #[response(status = 422)]
    UnprocessableEntity(Json<String>),
    #[response(status = 500)]
    InternalServerError(Json<String>),
    #[response(status = 404)]
    NotFound(Json<String>),
}

pub fn ok(message: String) -> CResponse {
    trace!("200: {}", message.to_string());
    return CResponse::Ok(Json(message));
}

pub fn bad_request(message: String) -> CResponse {
    warn!("400: {}", message.to_string());
    return CResponse::BadRequest(Json(format!(
        "{{\n \"status\": \"error\",\n \"code\": 400,\n \"message\": \"{}\"\n}}",
        message
    )));
}

pub fn unprocessable_entity(message: String) -> CResponse {
    warn!("422: {}", message.to_string());
    return CResponse::UnprocessableEntity(Json(format!(
        "{{\n \"status\": \"error\",\n \"code\": 422,\n \"message\": \"{}\"\n}}",
        message
    )));
}

pub fn internal_server_error(message: String) -> CResponse {
    warn!("500: {}", message.to_string());
    return CResponse::InternalServerError(Json(format!(
        "{{\n \"status\": \"error\",\n \"code\": 500,\n \"message\": \"{}\"\n}}",
        message
    )));
}

pub fn not_found(message: String) -> CResponse {
    warn!("404: {}", message.to_string());
    return CResponse::NotFound(Json(format!(
        "{{\n \"status\": \"error\",\n \"code\": 404,\n \"message\": \"{}\"\n}}",
        message
    )));
}