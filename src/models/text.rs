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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_drawtext() {
        let settings = TextSettings {
            font: "Arial".to_string(),
            size: 48,
            color: "white".to_string(),
            x: "(w-text_w)/2".to_string(),
            y: "100".to_string(),
        };
        let text = "Hello World";
        let expected = "drawtext=text='Hello World':fontfile='Arial':fontsize=48:fontcolor=white:x=(w-text_w)/2:y=100";
        assert_eq!(settings.to_drawtext(text), expected);
    }
}
