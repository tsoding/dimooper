use config::Config;

pub struct KeyboardLayoutScreen {
    config: Config
}

impl KeyboardLayoutScreen {
    pub fn new(config: Config) -> KeyboardLayoutScreen {
        KeyboardLayoutScreen {
            config: config
        }
    }
}
