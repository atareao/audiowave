use super::style::WaveformStyle;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WaveformSettings {
    pub width: u32,
    pub height: u32,
    pub x: String,
    pub y: String,
    pub style: Option<WaveformStyle>,
    pub pipeline: Option<Vec<String>>,
}

impl WaveformSettings {
    pub fn to_filter_chain(&self) -> String {
        // 1. Obtenemos el filtro base (del estilo o del primer paso del pipeline)
        let base_filter = if let Some(style) = &self.style {
            style.get_filter(self.width, self.height)
        } else {
            // Si no hay estilo, asumimos que el primer filtro del pipeline usa {w} y {h}
            let first = self
                .pipeline
                .as_ref()
                .and_then(|p| p.first())
                .cloned()
                .unwrap_or_else(|| "showwaves=s={w}x{h}".to_string());

            first
                .replace("{w}", &self.width.to_string())
                .replace("{h}", &self.height.to_string())
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
