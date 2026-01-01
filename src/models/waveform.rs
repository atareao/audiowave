use super::style::WaveformStyle;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WaveformSettings {
    pub width: u32,
    pub height: u32,
    pub x: String,
    pub y: String,
    pub style: Option<WaveformStyle>,
    pub color: Option<String>,
    pub pipeline: Option<Vec<String>>,
}

impl WaveformSettings {
    pub fn to_filter_chain(&self, rate: Option<i32>) -> String {
        // 1. Obtenemos el filtro base (del estilo o del primer paso del pipeline)
        let base_filter = if let Some(style) = &self.style {
            style.get_filter(self.width, self.height, self.color.as_deref(), rate)
        } else {
            // Si no hay estilo, asumimos que el primer filtro del pipeline usa {w} y {h}
            let first = self
                .pipeline
                .as_ref()
                .and_then(|p| p.first())
                .cloned()
                .unwrap_or_else(|| "showwaves=s={w}x{h}:rate={r}".to_string());

            let actual_rate = rate.unwrap_or(60);
            let mut replaced = first
                .replace("{w}", &self.width.to_string())
                .replace("{h}", &self.height.to_string());
            if let Some(color) = &self.color {
                replaced = replaced.replace("{c}", color);
            }
            replaced = replaced.replace("{r}", &actual_rate.to_string());
            replaced
        };

        // 2. Añadimos el resto de filtros del pipeline si existen
        if let Some(pipe) = &self.pipeline {
            // Si usamos estilo, el pipeline son pasos extra.
            // Si no usamos estilo, el primer paso ya lo hemos procesado.
            let skip_n = if self.style.is_some() { 0 } else { 1 };
            let extra_steps: Vec<String> = pipe.iter().skip(skip_n).cloned().collect();

            if extra_steps.is_empty() {
                base_filter
            } else {
                format!("{},{}", base_filter, extra_steps.join(","))
            }
        } else {
            base_filter
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::style::WaveformStyle;

    #[test]
    fn test_to_filter_chain_with_style_only() {
        let settings = WaveformSettings {
            width: 800,
            height: 300,
            x: "100".to_string(),
            y: "200".to_string(),
            style: Some(WaveformStyle::ClassicLine),
            color: None,
            pipeline: None,
        };
        let expected = "showwaves=s=800x300:mode=line:colors=cyan:rate=60,format=rgba,colorkey=0x000000:0.1:0.1";
        assert_eq!(settings.to_filter_chain(None), expected);
    }
    #[test]
    fn test_to_filter_chain_with_style_and_color() {
        let settings = WaveformSettings {
            width: 800,
            height: 300,
            x: "100".to_string(),
            y: "200".to_string(),
            style: Some(WaveformStyle::ClassicLine),
            color: Some("red".to_string()),
            pipeline: None,
        };
        let expected = "showwaves=s=800x300:mode=line:colors=red:rate=60,format=rgba,colorkey=0x000000:0.1:0.1";
        assert_eq!(settings.to_filter_chain(None), expected);
    }
    #[test]
    fn test_to_filter_chain_with_style_and_pipeline() {
        let settings = WaveformSettings {
            width: 800,
            height: 300,
            x: "100".to_string(),
            y: "200".to_string(),
            style: Some(WaveformStyle::ClassicLine),
            color: None,
            pipeline: Some(vec![
                "aformat=channel_layouts=mono".to_string(),
                "compand".to_string(),
            ]),
        };
        let expected = "showwaves=s=800x300:mode=line:colors=cyan:rate=60,format=rgba,colorkey=0x000000:0.1:0.1,aformat=channel_layouts=mono,compand";
        assert_eq!(settings.to_filter_chain(None), expected);
    }
    #[test]
    fn test_to_filter_chain_with_pipeline_only() {
        let settings = WaveformSettings {
            width: 800,
            height: 300,
            x: "100".to_string(),
            y: "200".to_string(),
            style: None,
            color: None,
            pipeline: Some(vec![
                "showwaves=s={w}x{h}:colors=red".to_string(),
                "compand".to_string(),
            ]),
        };
        let expected = "showwaves=s=800x300:colors=red,compand";
        assert_eq!(settings.to_filter_chain(None), expected);
    }
    #[test]
    fn test_to_filter_chain_with_pipeline_and_color() {
        let settings = WaveformSettings {
            width: 800,
            height: 300,
            x: "100".to_string(),
            y: "200".to_string(),
            style: None,
            color: Some("blue".to_string()),
            pipeline: Some(vec![
                "showwaves=s={w}x{h}:colors={c}".to_string(),
                "compand".to_string(),
            ]),
        };
        let expected = "showwaves=s=800x300:colors=blue,compand";
        assert_eq!(settings.to_filter_chain(None), expected);
    }

    #[test]
    fn test_to_filter_chain_with_no_style_no_pipeline() {
        let settings = WaveformSettings {
            width: 800,
            height: 300,
            x: "100".to_string(),
            y: "200".to_string(),
            style: None,
            color: None,
            pipeline: None,
        };
        let expected = "showwaves=s=800x300:rate=60";
        assert_eq!(settings.to_filter_chain(None), expected);
    }}
