use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")] // Permite escribir "classic_line" en el YAML
pub enum WaveformStyle {
    /// Una línea clásica que representa la onda de sonido.
    ClassicLine,
    /// Un espectro de frecuencias con estética cyberpunk.
    CyberpunkSpectrum,
    /// Simula la pantalla de un osciloscopio analógico.
    AnalogOscilloscope,
    /// Barras sólidas tipo ecualizador antiguo.
    RetroStep,
    /// Histograma de audio tipo cascada.
    NebulaHistogram,
    /// Espectro de frecuencia circular.
    PrismFrequency,
    /// Bloques de volumen dinámicos.
    DigitalPulse,
    /// Onda con reflejo inferior y brillo (glow).
    NeonMirror,
    /// Fondo translúcido con desenfoque gaussiano.
    GlassBlur,
    /// Estilo minimalista con rastro de movimiento (transparencia dinámica).
    GhostFrequency,
    /// Onda circular tipo Iron Man / HUD.
    CyberCircle,
    /// Estilo fluido con deformación y color cálido.
    LiquidGold,
    /// Rayos aleatorios basados en picos de frecuencia.
    ElectricStorm,
    /// Acumulación de espectro en 3D falso (estilo montaña).
    ZenithStack,
    /// Radar circular con barrido de frecuencia.
    PulseRadar,
    /// Barras simétricas desde el centro (tipo Audiogram).
    StudioBars,
    /// Línea única ultra-fina con degradado suave.
    MinimalMono,
    /// Área rellena (silhouette) con suavizado.
    WaveformSolid,
    /// Puntos que reaccionan sutilmente (estilo elegante).
    BroadcastPoint,
    /// Onda suave y contínua, ideal para la voz humana.
    TalkFlow,
    /// Estilo clásico de redes sociales con puntas redondeadas.
    AudiogramBars,
    /// Una silueta rellena que parece una sombra proyectada.
    VoiceShadow,
    /// Un círculo concéntrico sutil que rodea un elemento central.
    SpectrumCircle,
    /// Una onda moderna con un efecto de brillo.
    GlowWaveModern,
    /// Un ecualizador de 32 bandas.
    Equalizer32Bands,
    /// Una onda de sonido en forma circular.
    CircularWave,
    /// Una línea suave y continua, con un ligero desenfoque.
    SmoothLine,
}

impl WaveformStyle {
    pub fn get_filter(
        &self,
        width: u32,
        height: u32,
        color: Option<&str>,
        rate: Option<i32>,
    ) -> String {
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
                format!(
                    "avectorscope=s={width}x{height}:zoom=1.5:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::RetroStep => {
                let colors = color.unwrap_or("0x00FF00|0xFFFF00|0xFF0000");
                format!(
                    "showfreqs=s={width}x{height}:mode=bar:colors={colors}:fscale=log:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::NebulaHistogram => {
                let color = color.unwrap_or("rainbow");
                format!(
                    "ahistogram=s={width}x{height}:color={color}:scale=log:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::PrismFrequency => {
                let color = color.unwrap_or("magenta");
                format!(
                    "showwaves=s={width}x{height}:mode=p2p:colors={color}:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::DigitalPulse => {
                let color = color.unwrap_or("0xEE7611");
                format!(
                    "showvolume=w={width}:h={height}:f=0.9:c={color}:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::NeonMirror => {
                let color = color.unwrap_or("cyan");
                format!(
                    "showwaves=s={w}x{h}:mode=line:colors={color}:rate={actual_rate},format=rgba[wave_raw]; \
                     [wave_raw]split[fg][bg_glow]; \
                     [bg_glow]boxblur=10:5[glow]; \
                     [glow][fg]overlay=format=auto,colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height
                )
            }

            Self::GlassBlur => {
                let color = color.unwrap_or("white");
                format!(
                    "showfreqs=s={w}x{h}:mode=bar:colors={color}:rate={actual_rate},format=rgba[wave]; \
                     [wave]drawbox=t=fill:color=black@0.4,boxblur=luma_radius=10:luma_power=1[glass]; \
                     [glass][wave]overlay,colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height
                )
            }

            Self::GhostFrequency => {
                let color = color.unwrap_or("cyan");
                format!(
                    "showwaves=s={width}x{height}:mode=p2p:colors={color}:rate={actual_rate},lagfun=decay=0.95{rgba_colorkey}"
                )
            }

            Self::CyberCircle => {
                let color = color.unwrap_or("0x00FFFF");
                format!(
                    "showwaves=s={w}x{h}:mode=line:colors={color}:rate={actual_rate},format=rgba, \
                     polar=r=min(w\\,h)/2,colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height
                )
            }

            Self::LiquidGold => {
                let color = color.unwrap_or("0xFFD700");
                format!(
                    "showwaves=s={w}x{h}:mode=p2p:colors={color}:rate={actual_rate},format=rgba, \
                     boxblur=2:1,colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height
                )
            }

            Self::ElectricStorm => {
                let color = color.unwrap_or("0x8888FF");
                format!(
                    "showpeaks=s={w}x{h}:mode=line:color={color}:rate={actual_rate},format=rgba, \
                     boxblur=10:1,colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height
                )
            }

            Self::ZenithStack => {
                let color = color.unwrap_or("fire");
                format!(
                    "showspectrum=s={w}x{h}:mode=combined:color={color}:slide=scroll:fscale=log:rate={actual_rate},format=rgba, \
                     perspective=x0=0.2*W:y0=0:x1=0.8*W:y1=0:x2=0:y2=H:x3=W:y3=H,colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height
                )
            }

            Self::PulseRadar => {
                let colors = color.unwrap_or("0x00FF00");
                format!(
                    "showfreqs=s={w}x{h}:mode=bar:colors={colors}:rate={actual_rate},format=rgba, \
                     polar=r=min(w\\,h)/2,lagfun=decay=0.9,colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height
                )
            }

            Self::StudioBars => {
                let color = color.unwrap_or("white");
                format!(
                    "showwaves=s={width}x{height}:mode=cline:colors={color}:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::MinimalMono => {
                let color = color.unwrap_or("white");
                format!(
                    "showwaves=s={width}x{height}:mode=line:colors={color}:draw=full:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::WaveformSolid => {
                let color = color.unwrap_or("white@0.5");
                format!(
                    "showwaves=s={width}x{height}:mode=p2p:colors={color}:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::BroadcastPoint => {
                let color = color.unwrap_or("white@0.6");
                format!(
                    "showwaves=s={width}x{height}:mode=point:colors={color}:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::TalkFlow => {
                let color = color.unwrap_or("white");
                format!(
                    "showwaves=s={width}x{height}:mode=p2p:colors={color}:draw=full:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::AudiogramBars => {
                let color = color.unwrap_or("white");
                format!(
                    "showfreqs=s={width}x{height}:mode=bar:colors={color}:fscale=log:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::VoiceShadow => {
                let color = color.unwrap_or("white@0.3");
                format!(
                    "showwaves=s={width}x{height}:mode=cline:colors={color}:draw=full:rate={actual_rate}{rgba_colorkey}"
                )
            }

            Self::SpectrumCircle => {
                let color = color.unwrap_or("white");
                format!(
                    "showspectrum=s={w}x{h}:mode=combined:color={color}:slide=scroll:overlap=0.9:rate={actual_rate},format=rgba, \
                     polar=r=min(w\\,h)/2,colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height
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

            Self::Equalizer32Bands => {
                let color = color.unwrap_or("#ffffff");
                let freqs = [
                    20, 25, 31, 40, 50, 63, 80, 100, 125, 160, 200, 250, 315, 400, 500, 630, 800,
                    1000, 1250, 1500, 2000, 2500, 3150, 4000, 5000, 6300, 8000, 12000, 16000,
                    20000, 22000, 22050,
                ];

                let mut filter = String::new();

                // 1. Dividir el audio exactamente en el número de bandas (32)
                filter.push_str(&format!("asplit={}", freqs.len()));
                for i in 1..=freqs.len() {
                    filter.push_str(&format!("[AS{:02}]", i));
                }
                filter.push_str("; ");

                // 2. Definir dimensiones proporcionales
                // 32 barras + 33 huecos = 65 unidades de ancho
                let bar_w = width / 65;
                let gap_w = bar_w / 2; // Hueco más fino para resaltar las barras

                // 3. Generar cada banda (creciendo desde abajo)
                for (i, f) in freqs.iter().enumerate() {
                    let idx = i + 1;
                    filter.push_str(&format!(
                        "[AS{:02}]bandpass=f={}:w=4,showwaves=s={}x{}:mode=cline:r={}:colors={}[EQ{:02}]; ",
                        idx, f, bar_w, height, actual_rate, color, idx
                    ));
                }

                // 4. Crear los GAPs (espacios negros) con el rate configurado
                filter.push_str(&format!(
                    "color=s={}x{}:c=black:r={},split={}",
                    gap_w,
                    height,
                    actual_rate,
                    freqs.len() + 1
                ));
                for i in 1..=freqs.len() + 1 {
                    filter.push_str(&format!("[G{:02}]", i));
                }
                filter.push_str("; ");

                // 5. Construir la cadena de entrada para hstack (mapeo estricto)
                let mut hstack_inputs = String::new();
                for i in 1..=freqs.len() {
                    hstack_inputs.push_str(&format!("[G{:02}][EQ{:02}]", i, i));
                }
                hstack_inputs.push_str(&format!("[G{:02}]", freqs.len() + 1));

                // 6. Unión horizontal y transparencia final
                filter.push_str(&format!(
                    "{}hstack=inputs={}[BARS]; \
                     [BARS]format=rgba,colorkey=0x000000:0.1:0.1",
                    hstack_inputs,
                    freqs.len() * 2 + 1
                ));

                filter
            }

            Self::CircularWave => {
                let color = color.unwrap_or("white");
                format!(
                    "showwaves=s={w}x{h}:mode=cline:colors={c}:draw=full:rate={r},format=rgba,split[fill][border]; \
         [fill]colorchannelmixer=aa=0.3[fill_t]; \
         [fill_t][border]overlay=format=auto, \
         geq='p(mod(W/PI*(PI+atan2(H/2-Y,X-W/2)),W), H-2*hypot(H/2-Y,X-W/2))':\
         a='if(eq(alpha(mod(W/PI*(PI+atan2(H/2-Y,X-W/2)),W), H-2*hypot(H/2-Y,X-W/2)),0),0, \
            if(p(mod(W/PI*(PI+atan2(H/2-Y,X-W/2)),W), H-2*hypot(H/2-Y,X-W/2)),255,0))', \
         colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height,
                    c = color,
                    r = actual_rate
                )
            }
            Self::SmoothLine => {
                let color = color.unwrap_or("white");
                format!(
                    "showwaves=s={width}x{height}:mode=line:colors={color}:rate={actual_rate},format=rgba,colorkey=0x000000:0.1:0.1,boxblur=1"
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

    #[test]
    fn test_get_filter_equalizer_32_bands() {
        let style = WaveformStyle::Equalizer32Bands;
        let expected_filter = "asplit=32[AS01][AS02][AS03][AS04][AS05][AS06][AS07][AS08][AS09][AS10][AS11][AS12][AS13][AS14][AS15][AS16][AS17][AS18][AS19][AS20][AS21][AS22][AS23][AS24][AS25][AS26][AS27][AS28][AS29][AS30][AS31][AS32]; [AS01]bandpass=f=20:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ01]; [AS02]bandpass=f=25:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ02]; [AS03]bandpass=f=31:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ03]; [AS04]bandpass=f=40:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ04]; [AS05]bandpass=f=50:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ05]; [AS06]bandpass=f=63:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ06]; [AS07]bandpass=f=80:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ07]; [AS08]bandpass=f=100:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ08]; [AS09]bandpass=f=125:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ09]; [AS10]bandpass=f=160:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ10]; [AS11]bandpass=f=200:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ11]; [AS12]bandpass=f=250:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ12]; [AS13]bandpass=f=315:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ13]; [AS14]bandpass=f=400:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ14]; [AS15]bandpass=f=500:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ15]; [AS16]bandpass=f=630:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ16]; [AS17]bandpass=f=800:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ17]; [AS18]bandpass=f=1000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ18]; [AS19]bandpass=f=1250:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ19]; [AS20]bandpass=f=1500:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ20]; [AS21]bandpass=f=2000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ21]; [AS22]bandpass=f=2500:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ22]; [AS23]bandpass=f=3150:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ23]; [AS24]bandpass=f=4000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ24]; [AS25]bandpass=f=5000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ25]; [AS26]bandpass=f=6300:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ26]; [AS27]bandpass=f=8000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ27]; [AS28]bandpass=f=12000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ28]; [AS29]bandpass=f=16000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ29]; [AS30]bandpass=f=20000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ30]; [AS31]bandpass=f=22000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ31]; [AS32]bandpass=f=22050:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[EQ32]; color=s=9x720:c=black:r=60,split=33[G01][G02][G03][G04][G05][G06][G07][G08][G09][G10][G11][G12][G13][G14][G15][G16][G17][G18][G19][G20][G21][G22][G23][G24][G25][G26][G27][G28][G29][G30][G31][G32][G33]; [G01][EQ01][G02][EQ02][G03][EQ03][G04][EQ04][G05][EQ05][G06][EQ06][G07][EQ07][G08][EQ08][G09][EQ09][G10][EQ10][G11][EQ11][G12][EQ12][G13][EQ13][G14][EQ14][G15][EQ15][G16][EQ16][G17][EQ17][G18][EQ18][G19][EQ19][G20][EQ20][G21][EQ21][G22][EQ22][G23][EQ23][G24][EQ24][G25][EQ25][G26][EQ26][G27][EQ27][G28][EQ28][G29][EQ29][G30][EQ30][G31][EQ31][G32][EQ32][G33]hstack=inputs=65[BARS]; [BARS]format=rgba,colorkey=0x000000:0.1:0.1";
        assert_eq!(style.get_filter(1280, 720, None, None), expected_filter);
    }

    #[test]
    fn test_get_filter_circular_wave() {
        let style = WaveformStyle::CircularWave;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=cline:colors=white:draw=full:rate=60,format=rgba,split[fill][border]; [fill]colorchannelmixer=aa=0.3[fill_t]; [fill_t][border]overlay=format=auto, geq='p(mod(W/PI*(PI+atan2(H/2-Y,X-W/2)),W), H-2*hypot(H/2-Y,X-W/2))':a='if(eq(alpha(mod(W/PI*(PI+atan2(H/2-Y,X-W/2)),W), H-2*hypot(H/2-Y,X-W/2)),0),0, if(p(mod(W/PI*(PI+atan2(H/2-Y,X-W/2)),W), H-2*hypot(H/2-Y,X-W/2)),255,0))', colorkey=0x000000:0.1:0.1"
        );
    }

    #[test]
    fn test_get_filter_smooth_line() {
        let style = WaveformStyle::SmoothLine;
        assert_eq!(
            style.get_filter(100, 50, None, None),
            "showwaves=s=100x50:mode=line:colors=white:rate=60,format=rgba,colorkey=0x000000:0.1:0.1,boxblur=1"
        );
    }
}
