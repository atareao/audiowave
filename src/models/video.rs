use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VideoSettings {
    pub width: u32,
    pub height: u32,
    pub fps: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_settings() {
        let settings = VideoSettings {
            width: 1920,
            height: 1080,
            fps: 30,
        };

        assert_eq!(settings.width, 1920);
        assert_eq!(settings.height, 1080);
        assert_eq!(settings.fps, 30);
    }
}
