use rocket::{State};

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::response;

use crate::settings::{Settings};
use rocket::serde::json::Json;
use noreya_sdbp::sdbp::request::custom::io::IoBuilder;

use noreya_sdbp::sdbp::response::custom::io::output::OutputModeStatus;
use crate::input::json::output_pwm::{OutputPwmJson, MODE_PWM};
use crate::response::CResponse;

#[post("/io/<version>/<slot>/output/pwm",data="<param>")]
pub fn set_output_pwm(settings: &State<Settings>, version: ApiVersion, slot: u16,param : Json<OutputPwmJson>) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err,
    };

    let command : Vec<u8> ;
    let period_fix = param.period-1; // Note: This fixes the period offset
    command = match IoBuilder::new().output().set_output_pwm(param.pin,MODE_PWM, param.prescaler,param.time_on,period_fix) {
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
