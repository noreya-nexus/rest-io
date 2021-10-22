use rocket::{Request};
use rocket::response::content;

use crate::response;
use crate::response::CResponse;

#[catch(404)]
pub fn not_found<'a>(_req: &Request) -> CResponse {
    response::not_found("Not Found".to_string())
}

#[catch(422)]
pub fn unprocessable_entity<'a>(_req: &Request) -> CResponse {
    response::unprocessable_entity("Unprocessable Entity".to_string())
}

#[catch(500)]
pub fn internal_server_error<'a>(_req: &Request) -> CResponse {
    response::internal_server_error("Internal Server Error".to_string())
}

#[catch(502)]
pub fn bad_gateway(_req: &Request) -> content::Json<&'static str> {
    content::Json("{ \"status\": \"error\",\n \"code\": 502,\n \"message\": \"Bad Gateway\"\n }")
}

#[catch(400)]
pub fn bad_request<'a>(_req: &Request) -> CResponse {
    response::bad_request("Bad Request".to_string())
}