

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalInterruptJson {
    pub pin : u8,
    pub debounce_time_ms: u16,
    pub trigger: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalCounterJson {
    pub pin : u8,
    pub state : String,
}



