#![allow(dead_code)]
use crate::json::Validate;
use rocket::http::Status;


pub const MODE_OUTPUT : u8 = 0x01;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutputStateJson {
    pub pin : u8,
    pub state : String,
}

impl Validate for OutputStateJson {
    fn validate(&self) -> Result<(), Status> {

        if self.pin > 6 || self.pin < 1 {
            return Err(Status::UnprocessableEntity);
        }

        match map_state(&self.state) {
            Ok(_) => {}
            Err(_) => {return Err(Status::UnprocessableEntity)}
        }


        return Ok(());
    }
}

pub fn map_state(state: &String) -> Result<u8, Status>
{
    match state.as_str() {
        "low" => Ok(0),
        "high" => Ok(1),
        _ => Err(Status::UnprocessableEntity),
    }
}