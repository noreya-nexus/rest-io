#![allow(dead_code)]
use crate::json::Validate;
use rocket::http::Status;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputModeJson {
    pub pin : u8,
    pub mode: String,
}

impl InputModeJson {

    pub fn new(mode : String, pin : u8) -> InputModeJson {
        InputModeJson{ mode, pin }
    }

    pub fn create_default() -> InputModeJson {
        InputModeJson {mode : "analog".to_string() , pin : 0}
    }

    pub fn map_mode(mode: &String) -> Result<u8, Status> {
        match mode.as_str() {
            "analog" => Ok(1),
            "digital" => Ok(2),
            _ => Err(Status::UnprocessableEntity),
        }
    }
}

impl Validate for InputModeJson {
    fn validate(&self) -> Result<(), Status> {

        match self::InputModeJson::map_mode(&self.mode) {
            Ok(_) => {}
            Err(_) => { return Err(Status::UnprocessableEntity) }
        };

        if self.pin > 6 || self.pin < 1 {
            return Err(Status::UnprocessableEntity);
        }
        Ok(())
    }
}