mod models;
mod cli;

use clap::Parser;
use cli::Args;
use models::Config;
use std::fs;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parsear argumentos de la terminal
    let args = Args::parse();

    // 2. Cargar configuración YAML
    let yaml_content = fs::read_to_string(&args.config)
        .map_err(|_| format!("No se pudo encontrar el archivo de configuración: {}", args.config))?;
    let config: Config = serde_yaml::from_str(&yaml_content)?;

    // 3. Obtener la plantilla seleccionada
    let template = config.templates.get(&args.template)
        .ok_or_else(|| format!("La plantilla '{}' no existe en el YAML", args.template))?;

    // 4. Determinar textos (Lógica de prioridades)
    let title = args.title.unwrap_or_else(|| "Podcast Episode".to_string());
    let subtitle = args.subtitle.unwrap_or_else(|| "atareao.es".to_string());

    // 5. Construir el filtro complejo usando tus modelos
    let filter = template.build_filter_complex(&title, &subtitle);

    // 6. Configurar el comando FFmpeg
    let mut ffmpeg = Command::new("ffmpeg");
    ffmpeg
        .arg("-i").arg(&args.input)
        .arg("-i").arg(&template.background.path)
        .arg("-filter_complex").arg(filter)
        .arg("-map").arg("[outv]")
        .arg("-map").arg("0:a")
        .arg("-c:v").arg("libx264")
        .arg("-crf").arg("18")
        .arg("-pix_fmt").arg("yuv420p")
        .arg("-c:a").arg("aac")
        .arg("-shortest")
        .arg("-y"); // Sobrescribir salida

    // Si es preview, limitamos a 5 segundos
    if args.preview {
        ffmpeg.arg("-t").arg("5");
    }

    ffmpeg.arg(&args.output);

    println!("🚀 Generando video con la plantilla: {}...", args.template);
    
    let status = ffmpeg.status().await?;

    if status.success() {
        println!("✅ ¡Hecho! Video guardado en: {}", args.output);
    } else {
        eprintln!("❌ FFmpeg terminó con errores.");
    }

    Ok(())
}
