use serde::Deserialize;
use super::video::VideoSettings;
use super::background::BackgroundSettings;
use super::waveform::WaveformSettings;
use super::text::TextSettings;

#[derive(Debug, Deserialize)]
pub struct Template {
    pub video: VideoSettings,
    pub background: BackgroundSettings,
    pub waveform: WaveformSettings,
    pub title: TextSettings,
    pub subtitle: TextSettings,
}

impl Template {
    pub fn build_filter_complex(&self, title_text: &str, subtitle_text: &str) -> String {
        let bg_scale = self.background.to_filter(self.video.width, self.video.height);
        let wave_pipe = self.waveform.to_filter_chain();
        let draw_title = self.title.to_drawtext(title_text);
        let draw_subtitle = self.subtitle.to_drawtext(subtitle_text);

        format!(
            "[1:v]{bg_scale}[bg]; \
             [0:a]{wave_pipe}[wave_out]; \
             [bg][wave_out]overlay=x={wx}:y={wy}:shortest=1[v1]; \
             [v1]{title},{subtitle}[outv]",
            bg_scale = bg_scale,
            wave_pipe = wave_pipe,
            wx = self.waveform.x,
            wy = self.waveform.y,
            title = draw_title,
            subtitle = draw_subtitle
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::style::WaveformStyle;

    #[test]
    fn test_build_filter_complex() {
        let template = Template {
            video: VideoSettings {
                width: 1920,
                height: 1080,
                fps: 30,
            },
            background: BackgroundSettings {
                path: "background.png".to_string(),
                mode: "stretch".to_string(),
            },
            waveform: WaveformSettings {
                style: Some(WaveformStyle::ClassicLine),
                width: 800,
                height: 300,
                x: "100".to_string(),
                y: "200".to_string(),
                pipeline: None,
            },
            title: TextSettings {
                font: "Arial".to_string(),
                size: 64,
                color: "white".to_string(),
                x: "(w-text_w)/2".to_string(),
                y: "540".to_string(),
            },
            subtitle: TextSettings {
                font: "Arial".to_string(),
                size: 32,
                color: "white".to_string(),
                x: "(w-text_w)/2".to_string(),
                y: "600".to_string(),
            },
        };

        let filter = template.build_filter_complex("My Title", "My Subtitle");
        let expected = "[1:v]scale=1920:1080[bg]; [0:a]showwaves=s=800x300:mode=line:colors=cyan[wave_out]; [bg][wave_out]overlay=x=100:y=200:shortest=1[v1]; [v1]drawtext=text='My Title':fontfile='Arial':fontsize=64:fontcolor=white:x=(w-text_w)/2:y=540,drawtext=text='My Subtitle':fontfile='Arial':fontsize=32:fontcolor=white:x=(w-text_w)/2:y=600[outv]";
        assert_eq!(filter, expected);
    }
}
