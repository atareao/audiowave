use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")] // Permite escribir "classic_line" en el YAML
pub enum WaveformStyle {
    ClassicLine,
    CyberpunkSpectrum,
    AnalogOscilloscope,
    RetroStep,        // Barras sólidas tipo ecualizador antiguo
    NebulaHistogram,  // Histograma de audio tipo cascada
    PrismFrequency,   // Espectro de frecuencia circular
    DigitalPulse,     // Bloques de volumen dinámicos
    NeonMirror,      // Onda con reflejo inferior y brillo (glow)
    GlassBlur,       // Fondo translúcido con desenfoque gaussiano
    GhostFrequency,  // Estilo minimalista con rastro de movimiento (transparencia dinámica)
    CyberCircle,      // Onda circular tipo Iron Man / HUD
    LiquidGold,       // Estilo fluido con deformación y color cálido
    ElectricStorm,    // Rayos aleatorios basados en picos de frecuencia
    ZenithStack,      // Acumulación de espectro en 3D falso (estilo montaña)
    PulseRadar,       // Radar circular con barrido de frecuencia
    StudioBars,       // Barras simétricas desde el centro (tipo Audiogram)
    MinimalMono,      // Línea única ultra-fina con degradado suave
    WaveformSolid,    // Área rellena (silhouette) con suavizado
    BroadcastPoint,   // Puntos que reaccionan sutilmente (estilo elegante)
    TalkFlow,         // Onda suave y contínua, ideal para la voz humana
    AudiogramBars,    // Estilo clásico de redes sociales con puntas redondeadas
    VoiceShadow,      // Una silueta rellena que parece una sombra proyectada
    SpectrumCircle,   // Un círculo concéntrico sutil que rodea un elemento central
}

impl WaveformStyle {
    pub fn get_filter(&self, width: u32, height: u32) -> String {
        match self {
            WaveformStyle::ClassicLine => {
                format!("showwaves=s={width}x{height}:mode=line:colors=cyan")
            }

            WaveformStyle::CyberpunkSpectrum => {
                format!("showspectrum=s={width}x{height}:color=magma")
            }

            WaveformStyle::AnalogOscilloscope => {
                format!("avectorscope=s={width}x{height}:zoom=1.5")
            }

            WaveformStyle::RetroStep => {
                format!("showfreqs=s={width}x{height}:mode=bar:colors=0x00FF00|0xFFFF00|0xFF0000:fscale=log")
            }

            WaveformStyle::NebulaHistogram => {
                format!("ahistogram=s={width}x{height}:color=rainbow:scale=log")
            }

            WaveformStyle::PrismFrequency => {
                // Genera la frecuencia y luego la envuelve en coordenadas polares
                format!("showwaves=s={width}x{height}:mode=p2p:colors=magenta,format=yuv420p")
            }

            WaveformStyle::DigitalPulse => {
                format!("showvolume=w={width}:h={height}:f=0.9:c=0xEE7611")
            }

            WaveformStyle::NeonMirror => {
                // Genera la onda, la duplica, voltea una y le da opacidad/desenfoque
                format!(
                    "showwaves=s={w}x{h}:mode=line:colors=0xFF00FF[v]; \
                     [v]split[main][mirror]; \
                     [mirror]vflip,format=rgba,colorchannelmixer=aa=0.3,boxblur=5[m_final]; \
                     [main][m_final]vstack", 
                    w=width, h=height/2 // La onda ocupa la mitad para dejar espacio al reflejo
                )
            }

            WaveformStyle::GlassBlur => {
                // Crea un panel de "cristal" desenfocado detrás de la onda
                format!(
                    "showfreqs=s={w}x{h}:mode=bar:colors=white[wave]; \
                     [wave]format=rgba,drawbox=t=fill:color=black@0.4[bg_box]; \
                     [bg_box]boxblur=luma_radius=10:luma_power=1[glass]; \
                     [glass][wave]overlay",
                    w=width, h=height
                )
            }

            WaveformStyle::GhostFrequency => {
                // Usa el filtro 'lagfun' para que la onda deje un rastro transparente al moverse
                format!(
                    "showwaves=s={w}x{h}:mode=p2p:colors=cyan,format=rgba,lagfun=decay=0.95",
                    w=width, h=height
                )
            }

            WaveformStyle::CyberCircle => {
                // Genera la onda y la proyecta en coordenadas polares para hacerla circular
                format!(
                    "showwaves=s={w}x{h}:mode=line:colors=0x00FFFF, \
                     format=yuv420p, \
                     polar=r=min(w\\,h)/2:start=0:end=360",
                    w=width, h=height
                )
            }

            WaveformStyle::LiquidGold => {
                // Usa un filtro de desplazamiento (displace) para dar aspecto viscoso
                format!(
                    "showwaves=s={w}x{h}:mode=p2p:colors=0xFFD700, \
                     format=rgba, \
                     boxblur=2:1, \
                     colormatrix=bt709:fcc",
                    w=width, h=height
                )
            }

            WaveformStyle::ElectricStorm => {
                // Utiliza 'showpeaks' para generar rayos que saltan con el volumen
                format!(
                    "showpeaks=s={w}x{h}:mode=line:color=0x8888FF, \
                     drawbox=t=fill:color=0x0000FF@0.1, \
                     boxblur=10:1",
                    w=width, h=height
                )
            }

            WaveformStyle::ZenithStack => {
                // Acumula el espectro creando un efecto de profundidad 3D
                format!(
                    "showspectrum=s={w}x{h}:mode=combined:color=fire:slide=scroll:fscale=log, \
                     format=rgba, \
                     perspective=x0=0.2*W:y0=0:x1=0.8*W:y1=0:x2=0:y2=H:x3=W:y3=H",
                    w=width, h=height
                )
            }

            WaveformStyle::PulseRadar => {
                // Combinación de espectro circular con un rastro de desvanecimiento
                format!(
                    "showfreqs=s={w}x{h}:mode=bar:colors=0x00FF00, \
                     format=rgba, \
                     polar=r=min(w\\,h)/2, \
                     lagfun=decay=0.9" ,
                    w=width, h=height
                )
            }

            WaveformStyle::StudioBars => {
                // Barras que crecen hacia arriba y abajo desde el eje central
                format!(
                    "showwaves=s={w}x{h}:mode=cline:colors=0xFFFFFF@0.8:draw=full",
                    w=width, h=height
                )
            }

            WaveformStyle::MinimalMono => {
                // Una línea muy fina con un ligero rastro para que no parpadee
                format!(
                    "showwaves=s={w}x{h}:mode=line:colors=white:draw=full, \
                     format=rgba, \
                     boxblur=1:1" ,
                    w=width, h=height
                )
            }

            WaveformStyle::WaveformSolid => {
                // Relleno sólido de la onda, muy común en edición de audio profesional
                format!(
                    "showwaves=s={w}x{h}:mode=p2p:colors=white@0.5, \
                     format=rgba, \
                     fillcmd=fill",
                    w=width, h=height
                )
            }

            WaveformStyle::BroadcastPoint => {
                // Puntos discretos que flotan, dan movimiento sin saturar la imagen
                format!(
                    "showwaves=s={w}x{h}:mode=point:colors=white@0.6",
                    w=width, h=height
                )
            }

            WaveformStyle::TalkFlow => {
                // Una línea suave (p2p) con un filtro de promedio para evitar picos agresivos
                format!(
                    "showwaves=s={w}x{h}:mode=p2p:colors=white:draw=full, \
                     format=rgba, \
                     lowpass=f=500" ,
                    w=width, h=height
                )
            }

            WaveformStyle::AudiogramBars => {
                // Barras gruesas (mode=bar) con un ligero blur para simular bordes redondeados
                format!(
                    "showfreqs=s={w}x{h}:mode=bar:colors=0xFFFFFF@0.9:fscale=log, \
                     format=rgba, \
                     boxblur=2:2" ,
                    w=width, h=height
                )
            }

            WaveformStyle::VoiceShadow => {
                // Una onda rellena con opacidad baja, ideal para poner detrás de un logo o texto
                format!(
                    "showwaves=s={w}x{h}:mode=cline:colors=white@0.3:draw=full",
                    w=width, h=height
                )
            }

            WaveformStyle::SpectrumCircle => {
                // Espectro circular muy fino, perfecto para envolver la carátula del episodio
                format!(
                    "showspectrum=s={w}x{h}:mode=combined:color=white:slide=scroll:overlap=0.9, \
                     format=rgba, \
                     polar=r=min(w\\,h)/2" ,
                    w=width, h=height
                )
            }
        }
    }
}
