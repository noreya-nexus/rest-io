use rocket::{State};

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::response;
use crate::settings::{Settings};
use crate::input::json::power_config::{PwrConfigJson, map_config};
use rocket::serde::json::Json;
use nexus_unity_sdbp::sdbp::request::custom::io::IoBuilder;
use nexus_unity_sdbp::drv::api::*;

use std::convert::TryFrom;
use crate::response::CResponse;

#[post("/io/<version>/<slot>/power-management", data="<param>")]
pub fn powermgmt(settings: &State<Settings>, version: ApiVersion, slot: u16,param : Json<PwrConfigJson>) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err,
    };

    let result = com_manager.select_via_slot(0x2001);

    match result {
        Err(err) => return response::not_found(err.to_string()),
        _ => (),
    }

    let pins = map_config(&param.config);

    let mut frame = match IoBuilder::new().powermgmt().set_power_config(pins) {
        Err(err) => return response::bad_request(err.to_string()),
        Ok(value) => value,
    };

    let mut complete_frame : Vec<u8> = vec![slot as u8];
    complete_frame.append(&mut frame);
    let result = match com_manager.raw_command(complete_frame) {
        Ok(value) => value,
        Err(err) => return response::internal_server_error(err.to_string()),
    };

    let tmp = match result.get(&Tag::Response) {
        Some(value) => value,
        None => return response::internal_server_error("Unknown response received".to_string()),
    };


    let tmp = match TlvValue::try_from(tmp.as_bytes().unwrap().as_slice()) {
        Ok(value) => value,
        Err(_) => return response::internal_server_error("TLV Parsing failed".parse().unwrap()),
    };

    let tunnel = match tmp.get(&Tag::DeviceTunnel) {
        Some(value) => value,
        None => return response::internal_server_error("Unknown response received".to_string()),
    };

    let response = tunnel.get(&Tag::Response);
    let error_msg = tunnel.get(&Tag::ErrorMsg);

    if response.is_none() && error_msg.is_none()  {
        return response::internal_server_error("Unknown response received".to_string());
    }

    else if error_msg.is_some() {
        let msg = error_msg.unwrap().as_string();
        return response::unprocessable_entity(msg.unwrap().to_string());
    }
    response::ok("{ \"status\": \"success\" }".to_string())
}
