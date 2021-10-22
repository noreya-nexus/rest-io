use rocket::{State};

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::response;

use crate::settings::{Settings};
use rocket::serde::json::Json;
use nexus_unity_sdbp::sdbp::request::custom::io::IoBuilder;

use crate::input::json::output_state::{OutputStateJson, map_state, MODE_OUTPUT};
use nexus_unity_sdbp::sdbp::response::custom::io::output::OutputModeStatus;
use crate::response::CResponse;

#[post("/io/<version>/<slot>/output/state",data="<param>")]
pub fn set_output_state(settings: &State<Settings>, version: ApiVersion, slot: u16,param : Json<OutputStateJson>) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err,
    };

    let command : Vec<u8> ;

    command = match IoBuilder::new().output().set_output(param.pin,MODE_OUTPUT,map_state(&param.state).unwrap()) {
            Ok(value) => value,
            Err(err) => return response::bad_request(err.to_string()),
    };


    let result : Result<OutputModeStatus,std::io::Error> = com_manager.device_command(command);
    let status = match result {
        Ok(value) => value,
        Err(err) => return response::internal_server_error(err.to_string()),
    };


    if status.status != 0 {
         return response::bad_request(status.msg)
    }

    response::ok("{ \"status\": \"success\" }".to_string())
}
