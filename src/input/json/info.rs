use noreya_sdbp::datatypes::Version;


#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Root {
    pub driver_session: String,
    pub version: Driver,
    pub devices: Vec<Device>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Driver {
    pub api: Version,
    pub service: Version,
    pub module_driver: Version,
    pub kernel_driver: Version,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Device {
    pub slot_number: u32,
    pub sdbp_version: Version,
    pub device_session: String,
}

