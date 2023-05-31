#![allow(dead_code)]
use crate::json::Validate;
use rocket::http::Status;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResetJson {
    pub reset : String,
}

impl Validate for ResetJson {
    fn validate(&self) -> Result<(), Status> {

        if self.reset != "yes" {
            return Err(Status::BadRequest)
        }

        return Ok(());
    }
}
