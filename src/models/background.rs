use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BackgroundSettings {
    pub path: String,
    pub mode: String, // "stretch", "fit", "fill"
}

impl BackgroundSettings {
    pub fn to_filter(&self, v_width: u32, v_height: u32) -> String {
        match self.mode.as_str() {
            "fill" => format!("scale={w}:{h}:force_original_aspect_ratio=increase,crop={w}:{h}", w=v_width, h=v_height),
            "fit" => format!("scale={w}:{h}:force_original_aspect_ratio=decrease,pad={w}:{h}:(ow-iw)/2:(oh-ih)/2", w=v_width, h=v_height),
            _ => format!("scale={w}:{h}", w=v_width, h=v_height), // stretch
        }
    }
}
