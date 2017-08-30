use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::collections::HashMap;

use pm::DeviceInfo;
use serde_json;

use config::ConfigDeviceInfo;
use error::Result;

#[derive(Eq, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub last_input_port: Option<ConfigDeviceInfo>,
    pub last_output_port: Option<ConfigDeviceInfo>,
    pub keyboard_layout: HashMap<u64, u8>
}

impl Default for Config {
    fn default() -> Self {
        Config {
            last_input_port: None,
            last_output_port: None,
            keyboard_layout: HashMap::new(),
        }
    }
}

impl Config {
    pub fn load(file_path: &Path) -> Result<Config> {
        let mut serialized_config = String::new();
        let mut file = try!(fs::File::open(file_path));
        try!(file.read_to_string(&mut serialized_config));
        let config: Config = try!(serde_json::from_str(&serialized_config));
        Ok(config)
    }

    pub fn save(&self, file_path: &Path) -> Result<()> {
        let serialized_config: String = try!(serde_json::to_string(&self));
        let mut file = try!(fs::File::create(file_path));
        try!(file.write_all(serialized_config.as_bytes()));
        Ok(())
    }

    #[allow(dead_code)]
    pub fn update_last_ports(self,
                             input_port: DeviceInfo,
                             output_port: DeviceInfo) -> Config {
        Config {
            last_input_port: Some(ConfigDeviceInfo::new(&input_port)),
            last_output_port: Some(ConfigDeviceInfo::new(&output_port)),
            keyboard_layout: self.keyboard_layout
        }
    }
}
