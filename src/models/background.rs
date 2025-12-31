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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_filter_stretch() {
        let settings = BackgroundSettings {
            path: "test.png".to_string(),
            mode: "stretch".to_string(),
        };
        assert_eq!(settings.to_filter(1920, 1080), "scale=1920:1080");
    }

    #[test]
    fn test_to_filter_fit() {
        let settings = BackgroundSettings {
            path: "test.png".to_string(),
            mode: "fit".to_string(),
        };
        assert_eq!(settings.to_filter(1920, 1080), "scale=1920:1080:force_original_aspect_ratio=decrease,pad=1920:1080:(ow-iw)/2:(oh-ih)/2");
    }

    #[test]
    fn test_to_filter_fill() {
        let settings = BackgroundSettings {
            path: "test.png".to_string(),
            mode: "fill".to_string(),
        };
        assert_eq!(settings.to_filter(1920, 1080), "scale=1920:1080:force_original_aspect_ratio=increase,crop=1920:1080");
    }
}
