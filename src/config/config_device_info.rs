use pm::{DeviceInfo, PortMidiDeviceId};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigDeviceInfo {
    id: PortMidiDeviceId,
    name: String,
}

impl ConfigDeviceInfo {
    pub fn new(device_info: &DeviceInfo) -> ConfigDeviceInfo {
        ConfigDeviceInfo {
            id: device_info.id(),
            name: device_info.name().clone(),
        }
    }
}
