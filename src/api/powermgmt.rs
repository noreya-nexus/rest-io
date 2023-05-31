use rocket::{State};

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::{response, SharedData};
use crate::settings::{Settings};
use crate::input::json::power_config::{PwrConfigJson, map_config};
use rocket::serde::json::Json;
use noreya_sdbp::sdbp::request::custom::io::IoBuilder;
use noreya_sdbp::drv::api::*;

use std::convert::TryFrom;
use crate::response::CResponse;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct TooMuch {
    rail_3v3_milliwatt: u16,
    rail_5v0_milliwatt: u16,
    rail_12v_milliwatt: u16,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct PowerMgmtResponse {
    status: String,
    too_much_power: TooMuch
}

#[post("/io/<version>/<slot>/power-management", data="<param>")]
pub fn powermgmt(settings: &State<Settings>, version: ApiVersion, slot: u16,param : Json<PwrConfigJson>, shared: &State<SharedData>) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err,
    };

    let lock = shared.power_mgmt_lock.lock().expect("Could not lock mutex");

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
    else if response.is_some(){
        match response {
            None => {}
            Some(val) => {
                debug!("Power mgmt response: {:?}", val);
                match  val.as_bytes() {
                    None => {
                        return response::internal_server_error("Power management response is empty".to_string());
                    }
                    Some(val) => {
                        let too_much_3v3 = ((val[0] as u16) << 8) | val[1] as u16;
                        let too_much_5v0 = ((val[2] as u16) << 8) | val[3] as u16;
                        let too_much_12v = ((val[4] as u16) << 8) | val[5] as u16;

                        if too_much_3v3 != 0 || too_much_5v0 != 0 || too_much_12v !=0 {
                            let too_much_power = TooMuch {
                                rail_3v3_milliwatt: too_much_3v3,
                                rail_5v0_milliwatt: too_much_5v0,
                                rail_12v_milliwatt: too_much_12v
                            };
                            let response = PowerMgmtResponse{
                                status: "denied".to_string(),
                                too_much_power
                            };

                            let str = serde_json::to_string_pretty(&response).unwrap();
                            return response::ok(str)
                        }
                    }
                };
            }
        }
    }

    let too_much_power = TooMuch {
        rail_3v3_milliwatt: 0,
        rail_5v0_milliwatt: 0,
        rail_12v_milliwatt: 0
    };
    let mut response = PowerMgmtResponse{
        status: "success".to_string(),
        too_much_power: too_much_power.clone()
    };

    if *lock == true { // Ensure the lock is held
        response = PowerMgmtResponse { // This is useless but for the lock...
            status: "success".to_string(),
            too_much_power
        };
    }

    let str = serde_json::to_string_pretty(&response).unwrap();
    return response::ok(str);
}
