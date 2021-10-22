#![allow(dead_code)]
use crate::json::Validate;
use rocket::http::Status;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct AnalogThresholdJson {
    pub pin : u8,
    pub threshold_mV: u16,
    pub trigger: String,
}

impl AnalogThresholdJson {

    pub fn new(pin : u8, threshold_ms: u16, direction: String) -> AnalogThresholdJson {
        AnalogThresholdJson{ pin, threshold_mV: threshold_ms, trigger: direction }
    }

    pub fn create_default() -> AnalogThresholdJson {
        AnalogThresholdJson {pin : 0, threshold_mV: 0, trigger: "low".to_string()}
    }
}

impl Validate for AnalogThresholdJson {
    fn validate(&self) -> Result<(), Status> {
        Ok(())
    }
}