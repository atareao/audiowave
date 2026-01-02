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
    /// Un ecualizador de 128 bandas.
    Equalizer128Bands,
    /// Un ecualizador de 10 bandas.
    Equalizer10Bands,
    /// Una onda de sonido en forma circular.
    CircularWave,
    /// Una línea suave y continua, con un ligero desenfoque.
    SmoothLine,
    /// Tres capas de profundidad
    ProfessionalNeon,
    /// Efecto espejo con degradado
    CyberReflex,
    /// Espectrograma Circular Dinámico
    MagmaVortex,
    /// Partículas de espectro en Fuga
    Interstellar,
    /// Espejo Central
    VaporWaveMirror,
    /// Efecto Estela
    CyberGhost,
    ///Degradado vertical
    ToxicPulse,
    /// Circular con 128 bandas
    CircularNebula
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
                        "[AS{:02}]bandpass=f={}:w=4,showwaves=s={}x{}:mode=cline:r={}:colors={}[v{:02}]; \
                         [v{:02}]split[bg{:02}][fg{:02}]; \
                         [bg{:02}]boxblur=2:1[glow{:02}]; \
                         [glow{:02}][fg{:02}]overlay=format=auto[EQ{:02}]; ",
                        idx, f, bar_w, height, actual_rate, color,  idx, idx,
                        idx, idx, idx, idx, idx, idx, idx
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

            Self::Equalizer128Bands => {
                let color_input = color.unwrap_or("magma");

                format!(
                    "showfreqs=s={w}x{h}:mode=bar:fscale=log:ascale=log:colors={c}[mask]; \
                     [mask]scale=128:{h}:flags=neighbor,scale={w}:{h}:flags=neighbor[v_mask]; \
                     color=s={w}x{h}:c=blue,format=rgba[c1]; \
                     color=s={w}x{h}:c=magenta,format=rgba[c2]; \
                     color=s={w}x{h}:c=yellow,format=rgba[c3]; \
                     [c1][c2]blend=all_expr='A*(X/W)+B*(1-X/W)':shortest=1[grad1]; \
                     [grad1][c3]blend=all_expr='A*(X/W)+B*(1-X/W)':shortest=1[grad_final]; \
                     [grad_final][v_mask]blend=all_mode=multiply, \
                     split[main][glow]; \
                     [glow]boxblur=15:1,colorchannelmixer=aa=0.6[glow_f]; \
                     [glow_f][main]overlay=format=auto, \
                     colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height,
                    c = color_input,
                )
            }

            Self::Equalizer10Bands => {
                let color_input = color.unwrap_or("#ffffff");
                let freqs = [31, 63, 125, 250, 500, 1000, 2000, 4000, 8000, 16000];

                // Definimos el degradado para las 10 bandas (de graves a agudos)
                let gradient = [
                    "#00FF00", "#33FF00", "#66FF00", "#99FF00", "#CCFF00", "#FFFF00", "#FFCC00",
                    "#FF9900", "#FF6600", "#FF0000",
                ];

                let mut filter = String::new();

                // 1. Dividir audio
                filter.push_str(&format!("asplit={}", freqs.len()));
                for i in 1..=freqs.len() {
                    filter.push_str(&format!("[AS{:02}]", i));
                }
                filter.push_str("; ");

                let bar_w = width / 21;
                let gap_w = bar_w / 2;

                // 3. Generar bandas con degradado y glow
                for (i, f) in freqs.iter().enumerate() {
                    let idx = i + 1;
                    // Si el usuario pasó un color por parámetro, lo usamos.
                    // Si no, usamos el color del degradado para esa banda.
                    let bar_color = if color_input == "#ffffff" {
                        gradient[i]
                    } else {
                        color_input
                    };

                    filter.push_str(&format!(
                        "[AS{:02}]bandpass=f={}:w=4,showwaves=s={}x{}:mode=cline:r={}:colors={}[v{:02}]; \
                         [v{:02}]split[bg{:02}][fg{:02}]; \
                         [bg{:02}]boxblur=4:1[glow{:02}]; \
                         [glow{:02}][fg{:02}]overlay=format=auto[EQ{:02}]; ",
                        idx, f, bar_w, height, actual_rate, bar_color, idx, idx,
                        idx, idx, idx, idx, idx, idx, idx
                    ));
                }

                // 4. Espacios (GAPs)
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

                // 5. Mapeo para hstack
                let mut hstack_inputs = String::new();
                for i in 1..=freqs.len() {
                    hstack_inputs.push_str(&format!("[G{:02}][EQ{:02}]", i, i));
                }
                hstack_inputs.push_str(&format!("[G{:02}]", freqs.len() + 1));

                // 6. Unión horizontal y transparencia
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

            Self::ProfessionalNeon => {
                let color_main = color.unwrap_or("#00f2ff"); // Cyan neón
                let color_glow = "#0066ff"; // Azul eléctrico para el aura

                format!(
                    "asplit=2[a_wave][a_part]; \
                     [a_wave]showwaves=s={w}x{h}:mode=line:r={r}:colors={c_main}:draw=full,format=rgba[v_wave]; \
                     [v_wave]split[main][glow]; \
                     [glow]lutrgb=r=val:g=val:b=val,colorkey={c_glow}:0.1:0.1,boxblur=20:10,colorchannelmixer=aa=0.6[glow_f]; \
                     [glow_f][main]overlay=format=auto[combined]; \
                     [a_part]showfreqs=s={w}x{h}:mode=dot:colors={c_main}:fscale=log:rate={r},format=rgba, \
                     boxblur=2:1,colorchannelmixer=aa=0.4[part_f]; \
                     [combined][part_f]overlay=format=auto,colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height,
                    r = actual_rate,
                    c_main = color_main,
                    c_glow = color_glow
                )
            }

            Self::CyberReflex => {
                let c1 = color.unwrap_or("#00ff95"); // Verde neón
                format!(
                    "asplit=2[a1][a2]; \
         [a1]showwaves=s={w}x{h_half}:mode=cline:r={r}:colors={c}:draw=full,format=rgba[top]; \
         [a2]showwaves=s={w}x{h_half}:mode=cline:r={r}:colors={c}:draw=full,format=rgba,vflip,colorchannelmixer=aa=0.3[bottom]; \
         [top][bottom]vstack,format=rgba,colorkey=0x000000:0.1:0.1",
                    w = width,
                    h_half = height / 2,
                    r = actual_rate,
                    c = c1
                )
            }

            Self::MagmaVortex => {
                let c = color.unwrap_or("fire"); // Mapa de color 'fire', 'magma' o 'viridis'
                format!(
                    "showspectrum=s={w}x{h}:color={c}:mode=combined:slide=scroll:fscale=log:rate={r},format=rgba, \
                     polar=r=min(w\\,h)/2,colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height,
                    r = actual_rate,
                    c = c
                )
            }

            Self::Interstellar => {
                let c = color.unwrap_or("aqua");
                format!(
                    "ahistogram=s={w}x{h}:color={c}:scale=log:rate={r},format=rgba, \
         perspective=x0=0.3*W:y0=0:x1=0.7*W:y1=0:x2=0:y2=H:x3=W:y3=H, \
         colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height,
                    r = actual_rate,
                    c = c
                )
            }

            Self::VaporWaveMirror => {
                let c = color.unwrap_or("white");
                format!(
                    "showfreqs=s={w}x{h_half}:mode=bar:fscale=log:colors={c}[v]; \
         [v]scale=128:{h_half}:flags=neighbor,scale={w}:{h_half}:flags=neighbor,split[top][bot_pre]; \
         [bot_pre]vflip[bottom]; \
         [top][bottom]vstack[v_mask]; \
         color=s={w}x{h}:c=0x00FFFF,format=rgba[c1]; \
         color=s={w}x{h}:c=0xFF00FF,format=rgba[c2]; \
         [c1][c2]blend=all_expr='A*(X/W)+B*(1-X/W)'[grad]; \
         [grad][v_mask]blend=all_mode=multiply, \
         split[main][glow]; \
         [glow]boxblur=15:1,colorchannelmixer=aa=0.6[glow_f]; \
         [glow_f][main]overlay,colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height,
                    h_half = height / 2,
                    c = c
                )
            }

            Self::CyberGhost => {
                let c = color.unwrap_or("cyan");
                format!(
                    "showfreqs=s={w}x{h}:mode=bar:fscale=log:colors={c}[mask]; \
         [mask]scale=128:{h}:flags=neighbor,scale={w}:{h}:flags=neighbor,format=rgba, \
         tmix=frames=8:weights='1 0.8 0.6 0.4 0.2 0.1 0.05 0.02'[v_mask]; \
         color=s={w}x{h}:c=0x00FF00,format=rgba[c1]; \
         color=s={w}x{h}:c=0x003300,format=rgba[c2]; \
         [c1][c2]blend=all_expr='A*(1-Y/H)+B*(Y/H)'[grad]; \
         [grad][v_mask]blend=all_mode=multiply, \
         split[main][glow]; \
         [glow]boxblur=15:1,colorchannelmixer=aa=0.5[glow_f]; \
         [glow_f][main]overlay=format=auto, \
         colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height,
                    c = c,
                )
            }

            Self::ToxicPulse => {
                let c = color.unwrap_or("cyan");
                format!(
                    "showfreqs=s={w}x{h}:mode=bar:fscale=log:colors={c}[mask]; \
                     [mask]scale=128:{h}:flags=neighbor,scale={w}:{h}:flags=neighbor[v_mask]; \
                     color=s={w}x{h}:c=0xFF0000,format=rgba[red]; \
                     color=s={w}x{h}:c=0x00FF00,format=rgba[green]; \
                     [red][green]blend=all_expr='A*(1-Y/H)+B*(Y/H)'[grad_vert]; \
                     [grad_vert][v_mask]blend=all_mode=multiply, \
                     split[main][glow]; \
                     [glow]boxblur=20:1,colorchannelmixer=aa=0.8[glow_f]; \
                     [glow_f][main]overlay,colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height,
                    c = c
                )
            }

            Self::CircularNebula => {
                let c = color.unwrap_or("cyan");
                format!(
                    "showfreqs=s={w}x{h}:mode=bar:fscale=log:ascale=log:colors={c}[v]; \
                     [v]scale=128:{h}:flags=neighbor,scale={w}:{h}:flags=neighbor,format=rgba[mask]; \
                     color=s={w}x{h}:c=0x00CCFF,format=rgba[c1]; \
                     color=s={w}x{h}:c=0xFF00CC,format=rgba[c2]; \
                     [c1][c2]blend=all_expr='A*(X/W)+B*(1-X/W)':shortest=1[grad]; \
                     [grad][mask]blend=all_mode=multiply, \
                     geq='p(mod(W/PI*(PI+atan2(H/2-Y,X-W/2)),W), H-2*hypot(H/2-Y,X-W/2))', \
                     split[main][glow]; \
                     [glow]boxblur=12:1,colorchannelmixer=aa=0.6[glow_f]; \
                     [glow_f][main]overlay=format=auto, \
                     colorkey=0x000000:0.1:0.1",
                    w = width,
                    h = height,
                    c = c
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
        let expected_filter = "asplit=32[AS01][AS02][AS03][AS04][AS05][AS06][AS07][AS08][AS09][AS10][AS11][AS12][AS13][AS14][AS15][AS16][AS17][AS18][AS19][AS20][AS21][AS22][AS23][AS24][AS25][AS26][AS27][AS28][AS29][AS30][AS31][AS32]; [AS01]bandpass=f=20:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v01]; [v01]split[bg01][fg01]; [bg01]boxblur=2:1[glow01]; [glow01][fg01]overlay=format=auto[EQ01]; [AS02]bandpass=f=25:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v02]; [v02]split[bg02][fg02]; [bg02]boxblur=2:1[glow02]; [glow02][fg02]overlay=format=auto[EQ02]; [AS03]bandpass=f=31:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v03]; [v03]split[bg03][fg03]; [bg03]boxblur=2:1[glow03]; [glow03][fg03]overlay=format=auto[EQ03]; [AS04]bandpass=f=40:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v04]; [v04]split[bg04][fg04]; [bg04]boxblur=2:1[glow04]; [glow04][fg04]overlay=format=auto[EQ04]; [AS05]bandpass=f=50:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v05]; [v05]split[bg05][fg05]; [bg05]boxblur=2:1[glow05]; [glow05][fg05]overlay=format=auto[EQ05]; [AS06]bandpass=f=63:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v06]; [v06]split[bg06][fg06]; [bg06]boxblur=2:1[glow06]; [glow06][fg06]overlay=format=auto[EQ06]; [AS07]bandpass=f=80:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v07]; [v07]split[bg07][fg07]; [bg07]boxblur=2:1[glow07]; [glow07][fg07]overlay=format=auto[EQ07]; [AS08]bandpass=f=100:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v08]; [v08]split[bg08][fg08]; [bg08]boxblur=2:1[glow08]; [glow08][fg08]overlay=format=auto[EQ08]; [AS09]bandpass=f=125:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v09]; [v09]split[bg09][fg09]; [bg09]boxblur=2:1[glow09]; [glow09][fg09]overlay=format=auto[EQ09]; [AS10]bandpass=f=160:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v10]; [v10]split[bg10][fg10]; [bg10]boxblur=2:1[glow10]; [glow10][fg10]overlay=format=auto[EQ10]; [AS11]bandpass=f=200:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v11]; [v11]split[bg11][fg11]; [bg11]boxblur=2:1[glow11]; [glow11][fg11]overlay=format=auto[EQ11]; [AS12]bandpass=f=250:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v12]; [v12]split[bg12][fg12]; [bg12]boxblur=2:1[glow12]; [glow12][fg12]overlay=format=auto[EQ12]; [AS13]bandpass=f=315:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v13]; [v13]split[bg13][fg13]; [bg13]boxblur=2:1[glow13]; [glow13][fg13]overlay=format=auto[EQ13]; [AS14]bandpass=f=400:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v14]; [v14]split[bg14][fg14]; [bg14]boxblur=2:1[glow14]; [glow14][fg14]overlay=format=auto[EQ14]; [AS15]bandpass=f=500:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v15]; [v15]split[bg15][fg15]; [bg15]boxblur=2:1[glow15]; [glow15][fg15]overlay=format=auto[EQ15]; [AS16]bandpass=f=630:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v16]; [v16]split[bg16][fg16]; [bg16]boxblur=2:1[glow16]; [glow16][fg16]overlay=format=auto[EQ16]; [AS17]bandpass=f=800:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v17]; [v17]split[bg17][fg17]; [bg17]boxblur=2:1[glow17]; [glow17][fg17]overlay=format=auto[EQ17]; [AS18]bandpass=f=1000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v18]; [v18]split[bg18][fg18]; [bg18]boxblur=2:1[glow18]; [glow18][fg18]overlay=format=auto[EQ18]; [AS19]bandpass=f=1250:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v19]; [v19]split[bg19][fg19]; [bg19]boxblur=2:1[glow19]; [glow19][fg19]overlay=format=auto[EQ19]; [AS20]bandpass=f=1500:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v20]; [v20]split[bg20][fg20]; [bg20]boxblur=2:1[glow20]; [glow20][fg20]overlay=format=auto[EQ20]; [AS21]bandpass=f=2000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v21]; [v21]split[bg21][fg21]; [bg21]boxblur=2:1[glow21]; [glow21][fg21]overlay=format=auto[EQ21]; [AS22]bandpass=f=2500:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v22]; [v22]split[bg22][fg22]; [bg22]boxblur=2:1[glow22]; [glow22][fg22]overlay=format=auto[EQ22]; [AS23]bandpass=f=3150:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v23]; [v23]split[bg23][fg23]; [bg23]boxblur=2:1[glow23]; [glow23][fg23]overlay=format=auto[EQ23]; [AS24]bandpass=f=4000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v24]; [v24]split[bg24][fg24]; [bg24]boxblur=2:1[glow24]; [glow24][fg24]overlay=format=auto[EQ24]; [AS25]bandpass=f=5000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v25]; [v25]split[bg25][fg25]; [bg25]boxblur=2:1[glow25]; [glow25][fg25]overlay=format=auto[EQ25]; [AS26]bandpass=f=6300:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v26]; [v26]split[bg26][fg26]; [bg26]boxblur=2:1[glow26]; [glow26][fg26]overlay=format=auto[EQ26]; [AS27]bandpass=f=8000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v27]; [v27]split[bg27][fg27]; [bg27]boxblur=2:1[glow27]; [glow27][fg27]overlay=format=auto[EQ27]; [AS28]bandpass=f=12000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v28]; [v28]split[bg28][fg28]; [bg28]boxblur=2:1[glow28]; [glow28][fg28]overlay=format=auto[EQ28]; [AS29]bandpass=f=16000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v29]; [v29]split[bg29][fg29]; [bg29]boxblur=2:1[glow29]; [glow29][fg29]overlay=format=auto[EQ29]; [AS30]bandpass=f=20000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v30]; [v30]split[bg30][fg30]; [bg30]boxblur=2:1[glow30]; [glow30][fg30]overlay=format=auto[EQ30]; [AS31]bandpass=f=22000:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v31]; [v31]split[bg31][fg31]; [bg31]boxblur=2:1[glow31]; [glow31][fg31]overlay=format=auto[EQ31]; [AS32]bandpass=f=22050:w=4,showwaves=s=19x720:mode=cline:r=60:colors=#ffffff[v32]; [v32]split[bg32][fg32]; [bg32]boxblur=2:1[glow32]; [glow32][fg32]overlay=format=auto[EQ32]; color=s=9x720:c=black:r=60,split=33[G01][G02][G03][G04][G05][G06][G07][G08][G09][G10][G11][G12][G13][G14][G15][G16][G17][G18][G19][G20][G21][G22][G23][G24][G25][G26][G27][G28][G29][G30][G31][G32][G33]; [G01][EQ01][G02][EQ02][G03][EQ03][G04][EQ04][G05][EQ05][G06][EQ06][G07][EQ07][G08][EQ08][G09][EQ09][G10][EQ10][G11][EQ11][G12][EQ12][G13][EQ13][G14][EQ14][G15][EQ15][G16][EQ16][G17][EQ17][G18][EQ18][G19][EQ19][G20][EQ20][G21][EQ21][G22][EQ22][G23][EQ23][G24][EQ24][G25][EQ25][G26][EQ26][G27][EQ27][G28][EQ28][G29][EQ29][G30][EQ30][G31][EQ31][G32][EQ32][G33]hstack=inputs=65[BARS]; [BARS]format=rgba,colorkey=0x000000:0.1:0.1";
        assert_eq!(style.get_filter(1280, 720, None, None), expected_filter);
    }

    #[test]
    fn test_get_filter_equalizer_10_bands() {
        let style = WaveformStyle::Equalizer10Bands;
        let filter = style.get_filter(1260, 720, None, None);
        // 1260 / 21 = 60px por unidad. gap_w será 30px.
        assert!(filter.contains("asplit=10"));
        assert!(filter.contains("hstack=inputs=21"));
        assert!(filter.contains("color=s=30x720")); // gap_w (60/2)
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
