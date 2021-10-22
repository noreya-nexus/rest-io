#![allow(dead_code)]

use crate::json::Validate;
use rocket::http::Status;
use crate::settings::{MAX_PINS};

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PinConfigJson {
        pub rail: String,
        pub current: u16,
}

impl PinConfigJson {

    pub fn new(rail : String, current : u16) -> PinConfigJson {
        PinConfigJson{rail,current}
    }

    pub fn create_default() -> PinConfigJson {
        PinConfigJson { rail: "5_volt".to_string(), current: 0 }
    }
}

impl Validate for PinConfigJson {
    fn validate(&self) -> Result<(), Status> {
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PwrConfigJson {
    pub config: Vec<PinConfigJson>,
}

impl PwrConfigJson {

    pub fn new(config: Vec<PinConfigJson>) -> PwrConfigJson {
        PwrConfigJson{config}
    }

}

impl Validate for PwrConfigJson {
    fn validate(&self) -> Result<(), Status> {

        if self.config.len() != 6 {
            return Err(Status::UnprocessableEntity);
        }

        for element in &self.config {
            match map_rail(&element.rail) {
                Ok(_) => {}
                Err(_) => {return Err(Status::UnprocessableEntity)}
            }
        }

        Ok(())
    }
}

pub fn map_config(config: &Vec<PinConfigJson>) -> Vec<(u8,u16)> {
    let mut pin_pwr: Vec<(u8, u16)> = vec![];

    for _ in 0..MAX_PINS {
        pin_pwr.push((0, 0));
    }
    let mut cnt = 0;
    for pin in config {
        pin_pwr[cnt] = (map_rail(&pin.rail).unwrap(), pin.current);
        cnt += 1;
    }
    return pin_pwr;
}

pub fn map_rail(rail: &String) -> Result<u8, Status> {
    match rail.as_str() {
        "5_volt" => Ok(0),
        "12_volt" => Ok(1),
        _ => Err(Status::UnprocessableEntity),
    }
}