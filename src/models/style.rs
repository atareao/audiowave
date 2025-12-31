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
}

impl WaveformStyle {
    pub fn get_filter(&self, width: u32, height: u32) -> String {
        let rgba_colorkey = ",format=rgba,colorkey=0x000000:0.1:0.1";
        match self {
            WaveformStyle::ClassicLine => {
                format!("showwaves=s={width}x{height}:mode=line:colors=cyan:rate=30{rgba_colorkey}")
            }

            WaveformStyle::CyberpunkSpectrum => {
                format!("showspectrum=s={width}x{height}:color=magma:rate=30{rgba_colorkey}")
            }

            WaveformStyle::AnalogOscilloscope => {
                format!("avectorscope=s={width}x{height}:zoom=1.5:rate=30{rgba_colorkey}")
            }

            WaveformStyle::RetroStep => {
                format!("showfreqs=s={width}x{height}:mode=bar:colors=0x00FF00|0xFFFF00|0xFF0000:fscale=log{rgba_colorkey}")
            }

            WaveformStyle::NebulaHistogram => {
                format!("ahistogram=s={width}x{height}:color=rainbow:scale=log{rgba_colorkey}")
            }

            WaveformStyle::PrismFrequency => {
                format!("showwaves=s={width}x{height}:mode=p2p:colors=magenta:rate=30{rgba_colorkey}")
            }

            WaveformStyle::DigitalPulse => {
                format!("showvolume=w={width}:h={height}:f=0.9:c=0xEE7611{rgba_colorkey}")
            }

            WaveformStyle::NeonMirror => {
                format!(
                    "showwaves=s={w}x{h}:mode=line:colors=cyan:rate=30,format=rgba[wave_raw]; \
                     [wave_raw]split[fg][bg_glow]; \
                     [bg_glow]boxblur=10:5[glow]; \
                     [glow][fg]overlay=format=auto,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            WaveformStyle::GlassBlur => {
                format!(
                    "showfreqs=s={w}x{h}:mode=bar:colors=white:rate=30,format=rgba[wave]; \
                     [wave]drawbox=t=fill:color=black@0.4,boxblur=luma_radius=10:luma_power=1[glass]; \
                     [glass][wave]overlay,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            WaveformStyle::GhostFrequency => {
                format!("showwaves=s={width}x{height}:mode=p2p:colors=cyan:rate=30,lagfun=decay=0.95{rgba_colorkey}")
            }

            WaveformStyle::CyberCircle => {
                format!(
                    "showwaves=s={w}x{h}:mode=line:colors=0x00FFFF:rate=30,format=rgba, \
                     polar=r=min(w\\,h)/2,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            WaveformStyle::LiquidGold => {
                format!(
                    "showwaves=s={w}x{h}:mode=p2p:colors=0xFFD700:rate=30,format=rgba, \
                     boxblur=2:1,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            WaveformStyle::ElectricStorm => {
                format!(
                    "showpeaks=s={w}x{h}:mode=line:color=0x8888FF:rate=30,format=rgba, \
                     boxblur=10:1,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            WaveformStyle::ZenithStack => {
                format!(
                    "showspectrum=s={w}x{h}:mode=combined:color=fire:slide=scroll:fscale=log:rate=30,format=rgba, \
                     perspective=x0=0.2*W:y0=0:x1=0.8*W:y1=0:x2=0:y2=H:x3=W:y3=H,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            WaveformStyle::PulseRadar => {
                format!(
                    "showfreqs=s={w}x{h}:mode=bar:colors=0x00FF00:rate=30,format=rgba, \
                     polar=r=min(w\\,h)/2,lagfun=decay=0.9,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
                )
            }

            WaveformStyle::StudioBars => {
                format!("showwaves=s={width}x{height}:mode=cline:colors=white:rate=30{rgba_colorkey}")
            }

            WaveformStyle::MinimalMono => {
                format!("showwaves=s={width}x{height}:mode=line:colors=white:draw=full:rate=30{rgba_colorkey}")
            }

            WaveformStyle::WaveformSolid => {
                format!("showwaves=s={width}x{height}:mode=p2p:colors=white@0.5:rate=30{rgba_colorkey}")
            }

            WaveformStyle::BroadcastPoint => {
                format!("showwaves=s={width}x{height}:mode=point:colors=white@0.6:rate=30{rgba_colorkey}")
            }

            WaveformStyle::TalkFlow => {
                format!("showwaves=s={width}x{height}:mode=p2p:colors=white:draw=full:rate=30{rgba_colorkey}")
            }

            WaveformStyle::AudiogramBars => {
                format!("showfreqs=s={width}x{height}:mode=bar:colors=white:fscale=log:rate=30{rgba_colorkey}")
            }

            WaveformStyle::VoiceShadow => {
                format!("showwaves=s={width}x{height}:mode=cline:colors=white@0.3:draw=full:rate=30{rgba_colorkey}")
            }

            WaveformStyle::SpectrumCircle => {
                format!(
                    "showspectrum=s={w}x{h}:mode=combined:color=white:slide=scroll:overlap=0.9:rate=30,format=rgba, \
                     polar=r=min(w\\,h)/2,colorkey=0x000000:0.1:0.1",
                    w = width, h = height
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
            style.get_filter(100, 50),
            "showwaves=s=100x50:mode=line:colors=cyan:rate=30,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_cyberpunk_spectrum() {
        let style = WaveformStyle::CyberpunkSpectrum;
        assert_eq!(
            style.get_filter(100, 50),
            "showspectrum=s=100x50:color=magma:rate=30,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_analog_oscilloscope() {
        let style = WaveformStyle::AnalogOscilloscope;
        assert_eq!(style.get_filter(100, 50), "avectorscope=s=100x50:zoom=1.5:rate=30,format=rgba,colorkey=0x000000:0.1:0.1");
    }

    #[test]
    fn test_get_filter_retro_step() {
        let style = WaveformStyle::RetroStep;
        assert_eq!(
            style.get_filter(100, 50),
            "showfreqs=s=100x50:mode=bar:colors=0x00FF00|0xFFFF00|0xFF0000:fscale=log,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_nebula_histogram() {
        let style = WaveformStyle::NebulaHistogram;
        assert_eq!(
            style.get_filter(100, 50),
            "ahistogram=s=100x50:color=rainbow:scale=log,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_prism_frequency() {
        let style = WaveformStyle::PrismFrequency;
        assert_eq!(
            style.get_filter(100, 50),
            "showwaves=s=100x50:mode=p2p:colors=magenta:rate=30,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_digital_pulse() {
        let style = WaveformStyle::DigitalPulse;
        assert_eq!(
            style.get_filter(100, 50),
            "showvolume=w=100:h=50:f=0.9:c=0xEE7611,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_neon_mirror() {
        let style = WaveformStyle::NeonMirror;
        assert_eq!(
            style.get_filter(100, 50),
            "showwaves=s=100x50:mode=line:colors=cyan:rate=30,format=rgba[wave_raw]; [wave_raw]split[fg][bg_glow]; [bg_glow]boxblur=10:5[glow]; [glow][fg]overlay=format=auto,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_glass_blur() {
        let style = WaveformStyle::GlassBlur;
        assert_eq!(
            style.get_filter(100, 50),
            "showfreqs=s=100x50:mode=bar:colors=white:rate=30,format=rgba[wave]; [wave]drawbox=t=fill:color=black@0.4,boxblur=luma_radius=10:luma_power=1[glass]; [glass][wave]overlay,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_ghost_frequency() {
        let style = WaveformStyle::GhostFrequency;
        assert_eq!(
            style.get_filter(100, 50),
            "showwaves=s=100x50:mode=p2p:colors=cyan:rate=30,lagfun=decay=0.95,format=rgba,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_cyber_circle() {
        let style = WaveformStyle::CyberCircle;
        assert_eq!(
            style.get_filter(100, 50),
            "showwaves=s=100x50:mode=line:colors=0x00FFFF:rate=30,format=rgba, polar=r=min(w\\,h)/2,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_liquid_gold() {
        let style = WaveformStyle::LiquidGold;
        assert_eq!(
            style.get_filter(100, 50),
            "showwaves=s=100x50:mode=p2p:colors=0xFFD700:rate=30,format=rgba, boxblur=2:1,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_electric_storm() {
        let style = WaveformStyle::ElectricStorm;
        assert_eq!(
            style.get_filter(100, 50),
            "showpeaks=s=100x50:mode=line:color=0x8888FF:rate=30,format=rgba, boxblur=10:1,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_zenith_stack() {
        let style = WaveformStyle::ZenithStack;
        assert_eq!(
            style.get_filter(100, 50),
            "showspectrum=s=100x50:mode=combined:color=fire:slide=scroll:fscale=log:rate=30,format=rgba, perspective=x0=0.2*W:y0=0:x1=0.8*W:y1=0:x2=0:y2=H:x3=W:y3=H,colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_pulse_radar() {
        let style = WaveformStyle::PulseRadar;
        assert_eq!(
            style.get_filter(100, 50),
            "showfreqs=s=100x50:mode=bar:colors=0x00FF00, format=rgba, polar=r=min(w\\,h)/2, lagfun=decay=0.9"
        );
    }

    #[test]
    fn test_get_filter_studio_bars() {
        let style = WaveformStyle::StudioBars;
        assert_eq!(
            style.get_filter(100, 50),
            "showwaves=s=100x50:mode=cline:colors=0xFFFFFF@0.8:draw=full"
        );
    }

    #[test]
    fn test_get_filter_minimal_mono() {
        let style = WaveformStyle::MinimalMono;
        assert_eq!(
            style.get_filter(100, 50),
            "showwaves=s=100x50:mode=line:colors=white:draw=full, format=rgba, boxblur=1:1"
        );
    }

    #[test]
    fn test_get_filter_waveform_solid() {
        let style = WaveformStyle::WaveformSolid;
        assert_eq!(
            style.get_filter(100, 50),
            "showwaves=s=100x50:mode=p2p:colors=white@0.5, format=rgba, fillcmd=fill"
        );
    }

    #[test]
    fn test_get_filter_broadcast_point() {
        let style = WaveformStyle::BroadcastPoint;
        assert_eq!(
            style.get_filter(100, 50),
            "showwaves=s=100x50:mode=point:colors=white@0.6"
        );
    }

    #[test]
    fn test_get_filter_talk_flow() {
        let style = WaveformStyle::TalkFlow;
        assert_eq!(
            style.get_filter(100, 50),
            "showwaves=s=100x50:mode=p2p:colors=white:draw=full, format=rgba, lowpass=f=500"
        );
    }

    #[test]
    fn test_get_filter_audiogram_bars() {
        let style = WaveformStyle::AudiogramBars;
        assert_eq!(
            style.get_filter(100, 50),
            "showfreqs=s=100x50:mode=bar:colors=0xFFFFFF@0.9:fscale=log, format=rgba, boxblur=2:2"
        );
    }

    #[test]
    fn test_get_filter_voice_shadow() {
        let style = WaveformStyle::VoiceShadow;
        assert_eq!(
            style.get_filter(100, 50),
            "showwaves=s=100x50:mode=cline:colors=white@0.3:draw=full"
        );
    }

    #[test]
    fn test_get_filter_spectrum_circle() {
        let style = WaveformStyle::SpectrumCircle;
        assert_eq!(
            style.get_filter(100, 50),
            "showspectrum=s=100x50:mode=combined:color=white:slide=scroll:overlap=0.9, format=rgba, polar=r=min(w\\,h)/2"
        );
    }
}
