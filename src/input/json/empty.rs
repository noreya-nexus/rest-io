use crate::json::Validate;
use rocket::http::Status;
use rocket::serde::json::Json;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Param {}

impl Param {
    pub fn empty_json() -> Json<Param> {
        Json(Param{})
    }
}

impl Validate for Param {
    fn validate(&self) -> Result<(), Status> {
        Ok(())
    }
}