use noreya_sdbp::datatypes::{VersionError};
use rocket::request::FromParam;

pub struct ApiVersion {
    major: u16,
    minor: u16,
}

impl ApiVersion {
    pub fn major(&self) -> u16 {
        self.major
    }

    pub fn minor(&self) -> u16 {
        self.minor
    }

    fn from_str(input: &str) -> Result<ApiVersion, VersionError> {
        let mut buf: [u16; 2] = [0; 2];
        let mut idx: usize = 0;
        for token in input.split(".") {
            if idx == 2 || token.len() < 1 || token.len() > 5 {
                return Err(VersionError);
            }

            let value = match token.parse::<u16>() {
                Ok(value) => value,
                Err(_err) => return Err(VersionError),
            };

            buf[idx] = value;
            idx += 1;
        }

        if idx != 2 {
            return Err(VersionError);
        }
        Ok(ApiVersion { major: buf[0], minor: buf[1] })
    }
}

impl<'r> FromParam<'r> for ApiVersion {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match ApiVersion::from_str(&param.to_string()) {
            Ok(value) => Ok(ApiVersion { major: value.major(), minor: value.minor() }),
            Err(_err) => Err(param),
        }
    }
}

