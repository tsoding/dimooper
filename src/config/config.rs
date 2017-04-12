use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use pm::DeviceInfo;
use rustc_serialize::json;

use config::ConfigDeviceInfo;
use error::Result;

#[derive(Default, Debug, RustcDecodable, RustcEncodable)]
pub struct Config {
    last_input_port: Option<ConfigDeviceInfo>,
    last_output_port: Option<ConfigDeviceInfo>,
}

impl Config {
    pub fn load(file_path: &Path) -> Result<Config> {
        let mut serialized_config = String::new();
        let mut file = try!(fs::File::open(file_path));
        try!(file.read_to_string(&mut serialized_config));
        let config: Config = try!(json::decode(&serialized_config));
        Ok(config)
    }

    pub fn save(&self, file_path: &Path) -> Result<()> {
        let serialized_config: String = try!(json::encode(&self));
        let mut file = try!(fs::File::create(file_path));
        try!(file.write_all(serialized_config.as_bytes()));
        Ok(())
    }

    #[allow(dead_code)]
    pub fn update_last_ports(input_port: DeviceInfo,
                             output_port: DeviceInfo) -> Config {
        Config {
            last_input_port: Some(ConfigDeviceInfo::new(&input_port)),
            last_output_port: Some(ConfigDeviceInfo::new(&output_port))
        }
    }
}
