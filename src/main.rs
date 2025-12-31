mod cli;
mod models;

use clap::Parser;
use cli::Args;
use indicatif::{ProgressBar, ProgressStyle};
use models::{AudioMetadata, Config};
use regex::Regex;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};
use std::{
    error::Error,
    process::Stdio,
    collections::VecDeque
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let config_path = if args.config.is_empty() {
        None
    } else {
        Some(args.config.clone())
    };
    let config = Config::load(config_path).await?;
    let template = config
        .templates
        .get(&args.template)
        .ok_or_else(|| format!("Plantilla '{}' no encontrada", args.template))?;

    println!("🔍 Analizando archivo y metadatos...");
    let meta = AudioMetadata::new(args.input.clone()).await;

    let title = args.title.clone().unwrap_or(meta.title);
    let subtitle = args.subtitle.clone().unwrap_or(meta.artist);

    let output_file = if args.output == "output.mp4" || args.output.is_empty() {
        let safe_title = title.to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "_")
            .replace("__", "_");
        format!("{}.mp4", safe_title.trim_matches('_'))
    } else {
        args.output.clone()
    };

    let background = meta
        .cover_path
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| template.background.path.clone());

    let filter = template.build_filter_complex(&title, &subtitle);

    // --- Configuración de la Barra de Progreso ---
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}% ({eta})")?
            .progress_chars("#>-"),
    );

    // --- Lanzamiento de FFmpeg ---
    let mut child = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(&args.input)
        .arg("-i")
        .arg(&background)
        .arg("-filter_complex")
        .arg(filter)
        .arg("-map")
        .arg("[outv]")
        .arg("-map")
        .arg("0:a")
        .arg("-c:v")
        .arg("libx264")
        .arg("-preset")
        .arg("veryfast")
        .arg("-c:a")
        .arg("copy")
        .arg(&output_file)
        .stderr(Stdio::piped()) // Importante: capturamos stderr
        .spawn()?;

    let stderr = child
        .stderr
        .take()
        .ok_or("No se pudo capturar stderr de FFmpeg")?;
    let mut reader = BufReader::new(stderr).lines();

    // Regex para capturar Duration y time= de FFmpeg
    let re_duration = Regex::new(r"Duration: (\d{2}):(\d{2}):(\d{2})")?;
    let re_time = Regex::new(r"time=(\d{2}):(\d{2}):(\d{2})")?;

    let mut total_seconds = 0f64;
    // Búfer para guardar las últimas 15 líneas de log en caso de error
    let mut error_logs: VecDeque<String> = VecDeque::with_capacity(15);

    println!("🎬 Renderizando: {}...", title);

    // Bucle asíncrono para leer el progreso
    while let Ok(Some(line)) = reader.next_line().await {
        // Guardar línea en el búfer de logs (mantiene las últimas 15)
        if error_logs.len() >= 15 {
            error_logs.pop_front();
        }
        error_logs.push_back(line.clone());
        // 1. Intentar capturar la duración total al inicio
        if total_seconds == 0.0
            && let Some(caps) = re_duration.captures(&line)
        {
            let h: f64 = caps[1].parse()?;
            let m: f64 = caps[2].parse()?;
            let s: f64 = caps[3].parse()?;
            total_seconds = h * 3600.0 + m * 60.0 + s;
        }

        // 2. Capturar el tiempo actual de renderizado
        if let Some(caps) = re_time.captures(&line)
            && total_seconds > 0.0
        {
            let h: f64 = caps[1].parse()?;
            let m: f64 = caps[2].parse()?;
            let s: f64 = caps[3].parse()?;
            let current_seconds = h * 3600.0 + m * 60.0 + s;

            let percent = (current_seconds / total_seconds * 100.0) as u64;
            pb.set_position(percent);
        }
    }

    let status = child.wait().await?;
    pb.finish_with_message("¡Completado!");

    if status.success() {
        pb.finish_with_message("¡Completado!");
        println!("✅ Video guardado en: {}", args.output);
    } else {
        pb.abandon();
        eprintln!("\n❌ FFmpeg falló catastróficamente.");
        eprintln!("--- ÚLTIMOS LOGS DE ERROR ---");
        for log in error_logs {
            eprintln!("  > {}", log);
        }
        eprintln!("-----------------------------");
    }

    Ok(())
}
