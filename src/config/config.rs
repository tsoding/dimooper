use pm::DeviceInfo;
use std::path::Path;
use std::result::Result;
use config::ConfigDeviceInfo;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    last_input_port: Option<ConfigDeviceInfo>,
    last_output_port: Option<ConfigDeviceInfo>,
}

impl Config {
    fn default() -> Config {
        Config {
            last_input_port: None,
            last_output_port: None,
        }
    }

    // TODO: Implement loading and saving configuration

    fn new(file_path: &Path) -> Result<String, Config> {
        unimplemented!();
    }

    fn dump(&self, file_path: &Path) -> Result<String, ()> {
        unimplemented!();
    }

    fn update_last_ports(input_port: DeviceInfo,
                         output_port: DeviceInfo) -> Config {
        Config {
            last_input_port: Some(ConfigDeviceInfo::new(&input_port)),
            last_output_port: Some(ConfigDeviceInfo::new(&output_port))
        }
    }
}
