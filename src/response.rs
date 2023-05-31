#![allow(dead_code)]

use rocket::response::content::RawJson;
use rocket::response::Responder;

#[derive(Responder)]
pub enum CResponse {
    #[response(status = 200)]
    Ok(RawJson<String>),
    #[response(status = 400)]
    BadRequest(RawJson<String>),
    #[response(status = 422)]
    UnprocessableEntity(RawJson<String>),
    #[response(status = 500)]
    InternalServerError(RawJson<String>),
    #[response(status = 404)]
    NotFound(RawJson<String>),
}

pub fn ok(message: String) -> CResponse {
    trace!("200: {}", message.to_string());
    return CResponse::Ok(RawJson(message));
}

pub fn bad_request(message: String) -> CResponse {
    warn!("400: {}", message.to_string());
    return CResponse::BadRequest(RawJson(format!(
        "{{\n \"status\": \"error\",\n \"code\": 400,\n \"message\": \"{}\"\n}}",
        message
    )));
}

pub fn unprocessable_entity(message: String) -> CResponse {
    warn!("422: {}", message.to_string());
    return CResponse::UnprocessableEntity(RawJson(format!(
        "{{\n \"status\": \"error\",\n \"code\": 422,\n \"message\": \"{}\"\n}}",
        message
    )));
}

pub fn internal_server_error(message: String) -> CResponse {
    warn!("500: {}", message.to_string());
    return CResponse::InternalServerError(RawJson(format!(
        "{{\n \"status\": \"error\",\n \"code\": 500,\n \"message\": \"{}\"\n}}",
        message
    )));
}

pub fn not_found(message: String) -> CResponse {
    warn!("404: {}", message.to_string());
    return CResponse::NotFound(RawJson(format!(
        "{{\n \"status\": \"error\",\n \"code\": 404,\n \"message\": \"{}\"\n}}",
        message
    )));
}