use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")] // Permite escribir "classic_line" en el YAML
pub enum WaveformStyle {
    ClassicLine,
    CyberpunkSpectrum,
    AnalogOscilloscope,
    RetroStep,       // Barras sólidas tipo ecualizador antiguo
    NebulaHistogram, // Histograma de audio tipo cascada
    PrismFrequency,  // Espectro de frecuencia circular
    DigitalPulse,    // Bloques de volumen dinámicos
    NeonMirror,      // Onda con reflejo inferior y brillo (glow)
    GlassBlur,       // Fondo translúcido con desenfoque gaussiano
    GhostFrequency,  // Estilo minimalista con rastro de movimiento (transparencia dinámica)
    CyberCircle,     // Onda circular tipo Iron Man / HUD
    LiquidGold,      // Estilo fluido con deformación y color cálido
    ElectricStorm,   // Rayos aleatorios basados en picos de frecuencia
    ZenithStack,     // Acumulación de espectro en 3D falso (estilo montaña)
    PulseRadar,      // Radar circular con barrido de frecuencia
    StudioBars,      // Barras simétricas desde el centro (tipo Audiogram)
    MinimalMono,     // Línea única ultra-fina con degradado suave
    WaveformSolid,   // Área rellena (silhouette) con suavizado
    BroadcastPoint,  // Puntos que reaccionan sutilmente (estilo elegante)
    TalkFlow,        // Onda suave y contínua, ideal para la voz humana
    AudiogramBars,   // Estilo clásico de redes sociales con puntas redondeadas
    VoiceShadow,     // Una silueta rellena que parece una sombra proyectada
    SpectrumCircle,  // Un círculo concéntrico sutil que rodea un elemento central
    GlowWaveModern,
}

impl WaveformStyle {
    pub fn get_filter(&self, width: u32, height: u32, color: Option<&str>, rate: Option<i32>) -> String {
        let rgba_colorkey = ",format=rgba,colorkey=0x000000:0.1:0.1";
        let actual_rate = rate.unwrap_or(60);
        match self {
            Self::ClassicLine => {
                let color = color.unwrap_or("cyan");
                format!(
                    "showwaves=s={width}x{height}:mode=line:colors={color}:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::CyberpunkSpectrum => {
                let color = color.unwrap_or("magma");
                format!(
                    "showspectrum=s={width}x{height}:color={color}:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::AnalogOscilloscope => {
                format!("avectorscope=s={width}x{height}:zoom=1.5:rate={actual_rate}{rgba_colorkey}")
            }

            Self::RetroStep => {
                let colors = color.unwrap_or("0x00FF00|0xFFFF00|0xFF0000");
                format!("showfreqs=s={width}x{height}:mode=bar:colors={colors}:fscale=log:rate={actual_rate}{rgba_colorkey}")
            }

            Self::NebulaHistogram => {
                let color = color.unwrap_or("rainbow");
                format!("ahistogram=s={width}x{height}:color={color}:scale=log:rate={actual_rate}{rgba_colorkey}")
            }

            Self::PrismFrequency => {
                let color = color.unwrap_or("magenta");
                format!(
                    "showwaves=s={width}x{height}:mode=p2p:colors={color}:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::DigitalPulse => {
                let color = color.unwrap_or("0xEE7611");
                format!("showvolume=w={width}:h={height}:f=0.9:c={color}:rate={actual_rate}{rgba_colorkey}")
            }

            Self::NeonMirror => {
                let color = color.unwrap_or("cyan");
                format!(
                    "showwaves=s={w}x{h}:mode=line:colors={color}:rate={actual_rate},format=rgba[wave_raw]; \
                     [wave_raw]split[fg][bg_glow]; \
                     [bg_glow]boxblur=10:5[glow]; \
                     [glow][fg]overlay=format=auto,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            Self::GlassBlur => {
                let color = color.unwrap_or("white");
                format!(
                    "showfreqs=s={w}x{h}:mode=bar:colors={color}:rate={actual_rate},format=rgba[wave]; \
                     [wave]drawbox=t=fill:color=black@0.4,boxblur=luma_radius=10:luma_power=1[glass]; \
                     [glass][wave]overlay,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            Self::GhostFrequency => {
                let color = color.unwrap_or("cyan");
                format!("showwaves=s={width}x{height}:mode=p2p:colors={color}:rate={actual_rate},lagfun=decay=0.95{rgba_colorkey}")
            }

            Self::CyberCircle => {
                let color = color.unwrap_or("0x00FFFF");
                format!(
                    "showwaves=s={w}x{h}:mode=line:colors={color}:rate={actual_rate},format=rgba, \
                     polar=r=min(w\\,h)/2,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            Self::LiquidGold => {
                let color = color.unwrap_or("0xFFD700");
                format!(
                    "showwaves=s={w}x{h}:mode=p2p:colors={color}:rate={actual_rate},format=rgba, \
                     boxblur=2:1,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            Self::ElectricStorm => {
                let color = color.unwrap_or("0x8888FF");
                format!(
                    "showpeaks=s={w}x{h}:mode=line:color={color}:rate={actual_rate},format=rgba, \
                     boxblur=10:1,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            Self::ZenithStack => {
                let color = color.unwrap_or("fire");
                format!(
                    "showspectrum=s={w}x{h}:mode=combined:color={color}:slide=scroll:fscale=log:rate={actual_rate},format=rgba, \
                     perspective=x0=0.2*W:y0=0:x1=0.8*W:y1=0:x2=0:y2=H:x3=W:y3=H,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            Self::PulseRadar => {
                let colors = color.unwrap_or("0x00FF00");
                format!(
                    "showfreqs=s={w}x{h}:mode=bar:colors={colors}:rate={actual_rate},format=rgba, \
                     polar=r=min(w\\,h)/2,lagfun=decay=0.9,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            Self::StudioBars => {
                let color = color.unwrap_or("white");
                format!("showwaves=s={width}x{height}:mode=cline:colors={color}:rate={actual_rate}{rgba_colorkey}")
            }

            Self::MinimalMono => {
                let color = color.unwrap_or("white");
                format!("showwaves=s={width}x{height}:mode=line:colors={color}:draw=full:rate={actual_rate}{rgba_colorkey}")
            }

            Self::WaveformSolid => {
                let color = color.unwrap_or("white@0.5");
                format!("showwaves=s={width}x{height}:mode=p2p:colors={color}:rate={actual_rate}{rgba_colorkey}")
            }

            Self::BroadcastPoint => {
                let color = color.unwrap_or("white@0.6");
                format!("showwaves=s={width}x{height}:mode=point:colors={color}:rate={actual_rate}{rgba_colorkey}")
            }

            Self::TalkFlow => {
                let color = color.unwrap_or("white");
                format!("showwaves=s={width}x{height}:mode=p2p:colors={color}:draw=full:rate={actual_rate}{rgba_colorkey}")
            }

            Self::AudiogramBars => {
                let color = color.unwrap_or("white");
                format!("showfreqs=s={width}x{height}:mode=bar:colors={color}:fscale=log:rate={actual_rate}{rgba_colorkey}")
            }

            Self::VoiceShadow => {
                let color = color.unwrap_or("white@0.3");
                format!("showwaves=s={width}x{height}:mode=cline:colors={color}:draw=full:rate={actual_rate}{rgba_colorkey}")
            }

            Self::SpectrumCircle => {
                let color = color.unwrap_or("white");
                format!(
                    "showspectrum=s={w}x{h}:mode=combined:color={color}:slide=scroll:overlap=0.9:rate={actual_rate},format=rgba, \
                     polar=r=min(w\\,h)/2,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            Self::GlowWaveModern => {
                let color = color.unwrap_or("#00e5ff");
                format!(
                    "showwaves=s={width}x{height}:mode=cline:rate={actual_rate}:colors={color},format=rgba[wave_raw]; \
                     [wave_raw]format=rgba,split[wave1][wave2]; \
                     [wave2]boxblur=5:2[halo]; \
                     [halo][wave1]overlay=0:0"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_filter_classic_line() {
        let style = WaveformStyle::ClassicLine;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=line:colors=cyan:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }
    #[test]
    fn test_get_filter_classic_line_with_color() {
        let style = WaveformStyle::ClassicLine;
        assert_eq!(
            style.get_filter(100, 50, Some("red"), None),
            "showwaves=s=100x50:mode=line:colors=red:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_cyberpunk_spectrum() {
        let style = WaveformStyle::CyberpunkSpectrum;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showspectrum=s=100x50:color=magma:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_analog_oscilloscope() {
        let style = WaveformStyle::AnalogOscilloscope;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "avectorscope=s=100x50:zoom=1.5:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_retro_step() {
        let style = WaveformStyle::RetroStep;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showfreqs=s=100x50:mode=bar:colors=0x00FF00|0xFFFF00|0xFF0000:fscale=log:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_nebula_histogram() {
        let style = WaveformStyle::NebulaHistogram;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "ahistogram=s=100x50:color=rainbow:scale=log:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_prism_frequency() {
        let style = WaveformStyle::PrismFrequency;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=p2p:colors=magenta:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_digital_pulse() {
        let style = WaveformStyle::DigitalPulse;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showvolume=w=100:h=50:f=0.9:c=0xEE7611:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_neon_mirror() {
        let style = WaveformStyle::NeonMirror;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=line:colors=cyan:rate=60,format=rgba[wave_raw]; [wave_raw]split[fg][bg_glow]; [bg_glow]boxblur=10:5[glow]; [glow][fg]overlay=format=auto,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_glass_blur() {
        let style = WaveformStyle::GlassBlur;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showfreqs=s=100x50:mode=bar:colors=white:rate=60,format=rgba[wave]; [wave]drawbox=t=fill:color=black@0.4,boxblur=luma_radius=10:luma_power=1[glass]; [glass][wave]overlay,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_ghost_frequency() {
        let style = WaveformStyle::GhostFrequency;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=p2p:colors=cyan:rate=60,lagfun=decay=0.95,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_cyber_circle() {
        let style = WaveformStyle::CyberCircle;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=line:colors=0x00FFFF:rate=60,format=rgba, polar=r=min(w\\,h)/2,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_liquid_gold() {
        let style = WaveformStyle::LiquidGold;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=p2p:colors=0xFFD700:rate=60,format=rgba, boxblur=2:1,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_electric_storm() {
        let style = WaveformStyle::ElectricStorm;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showpeaks=s=100x50:mode=line:color=0x8888FF:rate=60,format=rgba, boxblur=10:1,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_zenith_stack() {
        let style = WaveformStyle::ZenithStack;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showspectrum=s=100x50:mode=combined:color=fire:slide=scroll:fscale=log:rate=60,format=rgba, perspective=x0=0.2*W:y0=0:x1=0.8*W:y1=0:x2=0:y2=H:x3=W:y3=H,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_pulse_radar() {
        let style = WaveformStyle::PulseRadar;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showfreqs=s=100x50:mode=bar:colors=0x00FF00:rate=60,format=rgba, \
                     polar=r=min(w\\,h)/2,lagfun=decay=0.9,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_studio_bars() {
        let style = WaveformStyle::StudioBars;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=cline:colors=white:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_minimal_mono() {
        let style = WaveformStyle::MinimalMono;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=line:colors=white:draw=full:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_waveform_solid() {
        let style = WaveformStyle::WaveformSolid;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=p2p:colors=white@0.5:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_broadcast_point() {
        let style = WaveformStyle::BroadcastPoint;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=point:colors=white@0.6:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_talk_flow() {
        let style = WaveformStyle::TalkFlow;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=p2p:colors=white:draw=full:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_audiogram_bars() {
        let style = WaveformStyle::AudiogramBars;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showfreqs=s=100x50:mode=bar:colors=white:fscale=log:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_voice_shadow() {
        let style = WaveformStyle::VoiceShadow;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=cline:colors=white@0.3:draw=full:rate=60,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_spectrum_circle() {
        let style = WaveformStyle::SpectrumCircle;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showspectrum=s=100x50:mode=combined:color=white:slide=scroll:overlap=0.9:rate=60,format=rgba, \
                     polar=r=min(w\\,h)/2,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_glow_wave_modern() {
        let style = WaveformStyle::GlowWaveModern;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=cline:rate=60:colors=#00e5ff,format=rgba[wave_raw]; \
                     [wave_raw]format=rgba,split[wave1][wave2]; \
                     [wave2]boxblur=5:2[halo]; \
                     [halo][wave1]overlay=0:0"
        );
    }
}
