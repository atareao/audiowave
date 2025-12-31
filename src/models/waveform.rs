use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WaveformSettings {
    pub width: u32,
    pub height: u32,
    pub x: String,
    pub y: String,
    pub pipeline: Vec<String>,
}

impl WaveformSettings {
    pub fn to_filter_chain(&self) -> String {
        self.pipeline.join(",")
            .replace("{w}", &self.width.to_string())
            .replace("{h}", &self.height.to_string())
    }
}
