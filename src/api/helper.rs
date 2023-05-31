use noreya_sdbp::drv::api::*;
use rocket::serde::json::Json;

use crate::api_version::ApiVersion;
use crate::input::json::Validate;
use crate::settings::Settings;
use std::time::Duration;
use crate::response;
use crate::response::CResponse;

pub struct Helper {}

impl Helper {
    pub fn init_api_device_command<T>(settings: &Settings, version: ApiVersion, slot: u16, param: &Json<T>) -> Result<Manager, CResponse> where T: Validate {
        if settings.api_version().major() != version.major() ||
            settings.api_version().minor() != version.minor()  {
            return Err(response::bad_request("Version not compatible".to_string()));
        }

        match param.validate() {
            Err(_err) => return Err(response::bad_request("Parameter validation failed".to_string())),
            Ok(_) => ()
        };

        let mut com_manager = match Manager::new(settings.socket_path(), Some(Duration::from_secs(10))) {
            Ok(value) => value,
            Err(_err) => return Err(response::internal_server_error("Could not connect to driver".to_string())),
        };

        match com_manager.select_via_slot(slot) {
            Err(_err) => return Err(response::not_found("Slot not found".to_string())),
            _ => ()
        }
        Ok(com_manager)
    }

    pub fn init_api<T>(settings: &Settings, version: ApiVersion, param: &Json<T>) -> Result<Manager, CResponse> where T: Validate {
        if settings.api_version().major() != version.major() ||
            settings.api_version().minor() != version.minor()  {
            return Err(response::bad_request("Version not compatible".to_string()));
        }

        match param.validate() {
            Err(_err) => return Err(response::bad_request("Parameter validation failed".to_string())),
            Ok(_) => ()
        };


        let com_manager = match Manager::new(settings.socket_path(), Some(Duration::from_secs(10))) {
            Ok(value) => value,
            Err(_err) => return Err(response::internal_server_error("Could not connect to driver".to_string())),
        };

        Ok(com_manager)
    }
}
