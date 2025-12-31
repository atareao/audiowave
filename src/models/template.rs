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
