pub const CONTROL_CHANNEL_NUMBER: u8 = 9;

pub const DEFAULT_TEMPO_BPM: u32 = 120;
pub const DEFAULT_MEASURE_SIZE_BPM: u32 = 4;
pub const DEFAULT_QUANTATION_LEVEL: u32 = 2;

pub const BEAT_KEY_NUMBER: u8 = 62;
pub const BEAT_VELOCITY: u8 = 60;
pub const BEAT_ACCENT_VELOCITY: u8 = 100;

pub const EVENT_LOOP_SLEEP_TIMEOUT: u64 = 3;
pub const CONTROL_KEY_NUMBER: u8 = 51;

pub const RATIO_WIDTH: u32 = 16;
pub const RATIO_HEIGHT: u32 = 9;
pub const RATIO_FACTOR: u32 = 90;

pub const AMOUNT_OF_MIDI_CHANNELS: usize = 16;
pub const AMOUNT_OF_MIDI_KEYS: usize = 128;

pub const TEMPO_CHANGE_CONTROL_NUMBER: u8 = 21;

pub const TTF_FONT_PATH: &'static str = "fonts/minoru.ttf";
pub const POPUP_FADEOUT_TIME: u32 = 500;
pub const POPUP_STAY_TIME: u32 = 500;

// TODO(be91501b-fd60-450d-b5c7-5ee4ad261c0c): make STATE_FILE_PATH a Path
//
// Consider using https://crates.io/crates/lazy_static
pub const STATE_FILE_PATH: &'static str = "state.json";

pub const CONFIG_FILE_NAME: &'static str = ".dimooper";

pub const KEYBOARD_MESSAGE_VELOCITY: u8 = 100;
pub const KEYBOARD_MESSAGE_CHANNEL: u8 = 2;
