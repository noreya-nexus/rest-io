use crate::json::Validate;
use rocket::http::Status;


pub(crate) const MODE_PWM : u8 = 0x02;
const BASE_PWM_FREQ: u32 = 48_000_000; // 48MHz
const MAX_PWM_FREQ: u32 = 1_000_000; // 1MHz

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutputPwmJson {
    pub pin : u8,
    pub prescaler : u16,
    pub time_on : u32,
    pub period : u32,
}


impl Validate for OutputPwmJson {
    fn validate(&self) -> Result<(), Status> {

        if self.pin > 6 || self.pin < 1 {
            return Err(Status::UnprocessableEntity);
        }

        if self.period < 2 {  // 2 means a duty cycle of minimum 50% is needed
            return Err(Status::UnprocessableEntity);
        }

        if self.time_on == 0 || self.time_on >= self.period-1 { // If time_on is 0 we have a steady high signal, time_on must be smaller than period
            return Err(Status::UnprocessableEntity);
        }

        if self.prescaler == 0 {
            return Err(Status::UnprocessableEntity);
        }

        if BASE_PWM_FREQ/(self.prescaler + 1) as u32/self.period > MAX_PWM_FREQ {
            return Err(Status::UnprocessableEntity);
        }

        return Ok(());
    }
}