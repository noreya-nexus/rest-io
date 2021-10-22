use rocket::{State};

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::response;
use crate::input::*;
use crate::settings::{Settings};

use nexus_unity_sdbp::sdbp::CoreBuilder;
use nexus_unity_sdbp::sdbp::response::core::notification::NotificationResponse;
use std::collections::HashMap;
use crate::response::CResponse;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct PinStatus {
    pub pin: u8,
    pub under_voltage_alert : bool,
    pub high_voltage_alert: bool,
    pub high_current_alert: bool,
    pub custom_voltage_alert : bool,
}

pub trait BoolStuff {
    fn to_bool(&self) -> bool;
}

impl BoolStuff for u8 {
    fn to_bool(&self) -> bool {
        if *self != 0 {
            return true;
        }
        return false;
    }
}

impl PinStatus {

    pub fn new(id : u8, hvolt : u8, uvolt : u8, hcur : u8, cusvolt :u8) -> PinStatus {
        PinStatus{ pin: id, under_voltage_alert: uvolt.to_bool(), high_voltage_alert : hvolt.to_bool(), high_current_alert: hcur.to_bool(), custom_voltage_alert : cusvolt.to_bool() }
    }
}

fn notification_to_string(notif: &Vec<u8>) -> Result<String,std::io::Error> {

    if notif.len() != 4 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Could not parse notification"));
    }
    trace!("Notification: {:x?}",notif);
    let mut pins = Vec::new();
    for i in 0..6 {
        let mask = 0x01 << i;
        let pin = PinStatus::new(i+1,notif[3] & mask, notif[2] & mask, notif[1] & mask, notif[0] & mask);
        pins.push(pin);
    }
    let mut response = HashMap::new();
    response.insert("notification", pins);
    return Ok(serde_json::to_string_pretty(&response).unwrap());
}


#[get("/io/<version>/<slot>/notification")]
pub fn get_notification(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err,
    };

    let command = CoreBuilder::new().notification().get_notification();

    let result : Result<NotificationResponse,std::io::Error> = com_manager.device_command(command);
    let response = match result {
        Ok(value) => value,
        Err(err) => return response::internal_server_error(err.to_string()),
    };

    return match notification_to_string(&response.notification) {
        Ok(value) => response::ok(value),
        Err(value) => response::bad_request(value.to_string())
    }
}
