#![allow(dead_code)]

use nexus_unity_sdbp::datatypes::Version;
use std::default::Default;


//IO Specific constrains
pub const MAX_PINS : u8 = 6;

// Set REST-API Version
pub const API_VERSION_MAJOR : u16 = 0;
pub const API_VERSION_MINOR : u16 = 9;
pub const API_VERSION_PATCH : u16 = 0;

pub const SOCKET_PATH : &str = "/var/run/nexus-drv-io/nexus-drv-io.socket";

#[derive(Clone,Debug)]
pub struct Settings {
    service_version : Version,
    api_version : Version,
    socket_path : String,
}

impl Settings {

    pub fn service_version(&self) -> Version {
        self.service_version.clone()
    }
    pub fn api_version(&self) -> Version {
        self.api_version.clone()
    }
    pub fn socket_path(&self) -> String {
        self.socket_path.clone()
    }
    pub fn max_pins(&self) -> u8 {MAX_PINS}
}

impl Default for Settings {
    fn default() -> Self {
       Settings{
           service_version : Version::from_str(env!("CARGO_PKG_VERSION")).unwrap(),
           api_version : Version::new(API_VERSION_MAJOR,API_VERSION_MINOR,API_VERSION_PATCH),
           socket_path : SOCKET_PATH.to_string(),
       }
    }
}

