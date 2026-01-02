use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Lorenzo Carbonell <atareao.es>", version, about = "Generador de videos con waveform a partir de MP3")]
pub struct Args {
    /// Archivo MP3 de entrada
    #[arg(short, long)]
    pub input: String,

    /// Nombre de la plantilla definida en el YAML
    #[arg(short, long, default_value = "default")]
    pub template: String,

    /// Título del video (sobreescribe los metadatos o el YAML)
    #[arg(short = 'm', long)]
    pub title: Option<String>,

    /// Subtítulo del video
    #[arg(short, long)]
    pub subtitle: Option<String>,

    /// Rate
    #[arg(short, long)]
    pub rate: Option<u32>,

    /// Color de la onda (ej. 'red', '#FF0000', '0xFF0000')
    #[arg(long)]
    pub wave_color: Option<String>,

    /// Archivo de salida (mkv)
    #[arg(short, long, default_value = "output.mkv")]
    pub output: String,

    /// Archivo de configuración YAML
    #[arg(short = 'c', long, default_value = "")]
    pub config: String,

    /// Modo de prueba: solo genera los primeros 5 segundos
    #[arg(short, long)]
    pub preview: bool,

    /// Activa el modo DEBUG
    #[arg(long)]
    pub debug: bool,
}
