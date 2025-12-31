use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VideoSettings {
    pub width: u32,
    pub height: u32,
    pub fps: u8,
}
