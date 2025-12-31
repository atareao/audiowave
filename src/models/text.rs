use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TextSettings {
    pub font: String,
    pub size: u32,
    pub color: String,
    pub x: String,
    pub y: String,
}

impl TextSettings {
    pub fn to_drawtext(&self, text: &str) -> String {
        format!(
            "drawtext=text='{text}':fontfile='{font}':fontsize={size}:fontcolor={color}:x={x}:y={y}",
            text = text,
            font = self.font,
            size = self.size,
            color = self.color,
            x = self.x,
            y = self.y
        )
    }
}
