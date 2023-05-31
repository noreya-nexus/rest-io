use rocket::{State};

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::{response, SharedData};
use crate::input::*;
use crate::settings::{Settings};

use noreya_sdbp::sdbp::CoreBuilder;
use noreya_sdbp::sdbp::response::core::notification::NotificationResponse;
use std::collections::HashMap;
use crate::response::CResponse;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PinStatus {
    #[serde(skip_serializing)]
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

fn merge_notification(old: &PinStatus, new: &PinStatus) -> Result<PinStatus,String> {

    if old.pin != new.pin {
        return Err("Pin numbers not matching!".to_string())
    }

    let mut merged = PinStatus {
        pin: old.pin,
        under_voltage_alert: old.under_voltage_alert,
        high_voltage_alert: old.high_voltage_alert,
        high_current_alert: old.high_current_alert,
        custom_voltage_alert: old.custom_voltage_alert
    };

    if old.under_voltage_alert == false && new.under_voltage_alert {
        merged.under_voltage_alert = true;
    }

    if old.high_voltage_alert == false && new.high_voltage_alert {
        merged.high_voltage_alert = true;
    }

    if old.high_current_alert == false && new.high_current_alert {
        merged.high_current_alert = true;
    }

    if old.custom_voltage_alert == false && new.custom_voltage_alert {
        merged.custom_voltage_alert = true;
    }

    return Ok(merged.clone());
}

fn update_notification_cache(notif: &Vec<u8>, shared: &State<SharedData>, slot: &u16) -> Result<String,std::io::Error> {

    if notif.len() != 4 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Could not parse notification"));
    }
    trace!("Notification: {:x?}",notif);

    let mut lock = shared.notifications.lock().expect("Could not lock mutex");
    let notification = match lock.get_mut(slot) {
        None => {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Could not find notification"));
        }
        Some(notification) => {
            for i in 0..notification.pins.len() as usize {
                let mask = 0x01 << i;
                let pin = PinStatus::new((i + 1) as u8, notif[3] & mask, notif[2] & mask, notif[1] & mask, notif[0] & mask);
                debug!("Old PinStatus: {:?}", notification.pins[i as usize]);
                debug!("New PinStatus: {:?}", pin);
                notification.pins[i as usize] = match merge_notification(&notification.pins[i as usize], &pin) {
                    Ok(value) => {
                        debug!("Merged: {:?}", value);
                        value }
                    Err(err) => { return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err)); }
                };
            }
            notification
        }
    };

    let mut response = HashMap::new();
    response.insert("notification", &notification.pins);
    return Ok(serde_json::to_string_pretty(&response).unwrap());
}

#[derive(Debug)]
pub struct Notifications {
    pub pins: Vec<PinStatus>
}

impl Notifications {
    pub fn new() -> Notifications {
        let mut pins = Vec::new();
        pins.push(PinStatus::new(1, 0, 0, 0, 0));
        pins.push(PinStatus::new(2, 0, 0, 0, 0));
        pins.push(PinStatus::new(3, 0, 0, 0, 0));
        pins.push(PinStatus::new(4, 0, 0, 0, 0));
        pins.push(PinStatus::new(5, 0, 0, 0, 0));
        pins.push(PinStatus::new(6, 0, 0, 0, 0));

        return Notifications {
            pins
        };
    }
}

#[get("/io/<version>/<slot>/notification/<pin>")]
pub fn get_notification(settings: &State<Settings>, version: ApiVersion, shared: &State<SharedData>, slot: u16, pin: u8) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err,
    };

    if pin < 1 || pin > 6 {
        return response::not_found("Pin out of range".to_string())
    }

    let command = CoreBuilder::new().notification().get_notification();

    let result : Result<NotificationResponse,std::io::Error> = com_manager.device_command(command);
    let response = match result {
        Ok(value) => value,
        Err(err) => return response::internal_server_error(err.to_string()),
    };

    { // Unlock at the end of this block
        let mut lock = shared.notifications.lock().expect("Could not lock mutex");

        if lock.is_empty(){
            // Initialize here (not possible in main because of async code)
            for slot in 0..10 {
                lock.insert(slot, Notifications::new());
            }
        }
    }
    debug!("Notification: {:?}", response.notification);
    match update_notification_cache(&response.notification, shared, &slot) {
        Ok(_) => {},
        Err(err) => return response::internal_server_error(err.to_string()),
    };

    let mut lock = shared.notifications.lock().expect("Could not lock mutex");
    match lock.get_mut(&slot) {
        None => {
            return response::internal_server_error("Could not find notification".to_string());
        }
        Some(notification) => {
            for i in 0..notification.pins.len() as usize {
                if i == (pin-1) as usize {
                    let value = &notification.pins[i as usize].clone();
                    notification.pins[i as usize] = PinStatus::new((i+1) as u8,0,0,0,0);
                    return response::ok(serde_json::to_string_pretty(value).unwrap());
                }
            }
        }
    }
    return response::internal_server_error("Could not find pin".to_string());

}
