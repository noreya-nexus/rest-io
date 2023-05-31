use rocket::{ State};

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::{response, SharedData};
use crate::input::*;
use crate::settings::Settings;
use crate::response::CResponse;

#[get("/io/<version>/<slot>/descriptor")]
pub fn get_descriptor(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {
    let mut com_manager = match Helper::init_api(&settings, version, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err,
    };

    let result = com_manager.get_device_list(false);

    match result {
        Ok(device_list) => {
            for entry in device_list {
                if entry.adr() == slot {
                    return response::ok(serde_json::to_string_pretty(&entry).unwrap());
                }
            }
        },
        Err(_err) => {
            return response::internal_server_error("Could not get device list".to_string());
        },
    };
    response::not_found("Could not find device on this slot".to_string())
}

#[get("/io/<version>")]
pub fn index(settings: &State<Settings>, shared: &State<SharedData>, version: ApiVersion) -> CResponse {
    let mut com_manager = match Helper::init_api(&settings, version, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err,
    };
    let lock = shared.driver_session.lock().expect("Could not lock mutex");

    let mut js_dev_list: Vec<json::info::Device> = vec![];
    let drv_info = match com_manager.get_info() {
        Ok(drv_info) => {drv_info},
        Err(_) => return response::internal_server_error("Failed getting device info".to_string()),
    };

    let result = com_manager.get_device_list(true);

    match result {
        Ok(device_list) => {
            for entry in device_list {
                let test = json::info::Device { slot_number: (entry.adr() as u32),
                    sdbp_version: entry.protocol_version().clone(),
                    device_session: entry.device_session().clone() };
                js_dev_list.push(test);
            }
        },
        Err(_err) => {
            return response::internal_server_error("Could not get device list".to_string());
        },
    };
    let module_driver = drv_info.clone().get_version();
    let kernel_driver = drv_info.clone().get_sdbpk_version();
    let res_json = json::info::Root {
        driver_session: lock.to_ascii_uppercase(),
        version: json::info::Driver {
            api: settings.api_version(),
            service: settings.service_version(),
            module_driver,
            kernel_driver
        },
        devices: js_dev_list,
    };
    response::ok(serde_json::to_string_pretty(&res_json).unwrap())
}
