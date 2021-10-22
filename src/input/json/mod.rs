pub mod empty;
pub mod info;

pub mod power_config;
pub mod inputmode;
pub mod analog_threshold;
pub mod input;
pub mod output_state;
pub mod output_pwm;

use rocket::http::Status;

pub trait Validate {
    fn validate(&self) -> Result<(),Status>;
}