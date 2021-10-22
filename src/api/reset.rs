use rocket::{State};

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::response;
use crate::input::*;
use crate::settings::{Settings};

use nexus_unity_sdbp::sdbp::CoreBuilder;
use nexus_unity_sdbp::sdbp::response::core::control::{SuspendResponse,RunResponse};
use crate::response::CResponse;

#[get("/io/<version>/<slot>/reset")]
pub fn module_reset(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err,
    };

    let command = CoreBuilder::new().control().mode_suspend();

    let result : Result<SuspendResponse,std::io::Error> = com_manager.device_command(command.expect("Internal error"));
    match result {
        Ok(_value) => (),
        Err(err) => return response::internal_server_error(err.to_string()),
    };

    let command = CoreBuilder::new().control().mode_run();
    let result : Result<RunResponse,std::io::Error> = com_manager.device_command(command.expect("Internal error"));
    let response = match result {
        Ok(_value) => response::ok("{ \"status\": \"success\" }".to_string()),
        Err(err) => response::internal_server_error(err.to_string()),
    };

    return response;
}
