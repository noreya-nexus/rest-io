use rocket::{State};

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::response;
use crate::input::*;
use crate::settings::{Settings};
use rocket::serde::json::Json;
use nexus_unity_sdbp::sdbp::request::custom::io::IoBuilder;
use nexus_unity_sdbp::sdbp::response::custom::io::input::*;

use crate::input::json::inputmode::{InputModeJson};
use crate::input::json::analog_threshold::AnalogThresholdJson;
use crate::input::json::input::DigitalInterruptJson;
use crate::input::json::input::DigitalCounterJson;
use crate::response::CResponse;

#[post("/io/<version>/<slot>/input/mode",data="<param>")]
pub fn set_input_mode(settings: &State<Settings>, version: ApiVersion, slot: u16,param : Json<InputModeJson>) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err,
    };

    let command = match IoBuilder::new().input().set_input_mode(param.pin,InputModeJson::map_mode(&param.mode).unwrap()) {
        Ok(value) => value,
        Err(err) => return response::bad_request(err.to_string()),
    };


    let result : Result<InputModeStatus,std::io::Error> = com_manager.device_command(command);
    let status = match result {
        Ok(value) => value,
        Err(err) => return response::internal_server_error(err.to_string()),
    };


    if status.status != 0 {
         return response::bad_request(status.msg)
    }
    response::ok("{ \"status\": \"success\" }".to_string())
}

#[post("/io/<version>/<slot>/input/analog/threshold",data="<param>")]
pub fn set_analog_threshold(settings: &State<Settings>, version: ApiVersion, slot: u16,param : Json<AnalogThresholdJson>) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err,
    };

    let command = match IoBuilder::new().input().set_analog_threshold(param.pin, param.threshold_mV, &param.trigger) {
        Ok(value) => value,
        Err(err) => return response::bad_request(err.to_string()),
    };


    let result : Result<AnalogThresholdStatus,std::io::Error> = com_manager.device_command(command);
    let status = match result {
        Ok(value) => value,
        Err(err) => return response::internal_server_error(err.to_string()),
    };

    if status.status != 0 {
        return response::bad_request(status.msg)
    }
    response::ok("{ \"status\": \"success\" }".to_string())
}

#[post("/io/<version>/<slot>/input/digital/interrupt",data="<param>")]
pub fn set_digital_interrupt(settings: &State<Settings>, version: ApiVersion, slot: u16, param : Json<DigitalInterruptJson>) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err,
    };

    let command = match IoBuilder::new().input().set_digital_interrupt(param.pin, param.debounce_time_ms, &param.trigger) {
        Ok(value) => value,
        Err(err) => return response::bad_request(err.to_string()),
    };


    let result : Result<DigitalInterruptStatus,std::io::Error> = com_manager.device_command(command);
    let status = match result {
        Ok(value) => value,
        Err(err) => return response::internal_server_error(err.to_string()),
    };

    if status.status != 0 {
        return response::bad_request(status.msg)
    }
    response::ok("{ \"status\": \"success\" }".to_string())
}

#[post("/io/<version>/<slot>/input/digital/counter",data="<param>")]
pub fn set_digital_counter(settings: &State<Settings>, version: ApiVersion, slot: u16,param : Json<DigitalCounterJson>) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err,
    };

    let command = match IoBuilder::new().input().set_digital_counter(param.pin,&param.state) {
        Ok(value) => value,
        Err(err) => return response::bad_request(err.to_string()),
    };


    let result : Result<DigitalCounterStatus,std::io::Error> = com_manager.device_command(command);
    let status = match result {
        Ok(value) => value,
        Err(err) => return response::internal_server_error(err.to_string()),
    };

    if status.status != 0 {
        return response::bad_request(status.msg)
    }
    response::ok("{ \"status\": \"success\" }".to_string())
}

#[get("/io/<version>/<slot>/input/values")]
pub fn get_values(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err,
    };

    let command = match IoBuilder::new().input().get_values() {
        Ok(value) => value,
        Err(err) => return response::internal_server_error(err.to_string()),
    };


    let result : Result<GetValuesStatus,std::io::Error> = com_manager.device_command(command);
    let response = match result {
        Ok(value) => value,
        Err(err) => return response::internal_server_error(err.to_string()),
    };

    let str = serde_json::to_string_pretty(&response).unwrap();
    response::ok(str)
}

#[get("/io/<version>/<slot>/input/values/current")]
pub fn get_current_values(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err,
    };

    let command = match IoBuilder::new().input().get_current_values() {
        Ok(value) => value,
        Err(err) => return response::internal_server_error(err.to_string()),
    };


    let result : Result<GetValuesStatus,std::io::Error> = com_manager.device_command(command);
    let response = match result {
        Ok(value) => value,
        Err(err) => return response::internal_server_error(err.to_string()),
    };

    let str = serde_json::to_string_pretty(&response).unwrap();
    response::ok(str)

}
