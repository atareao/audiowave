use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;
use std::fs::File;
use std::io::Write;
use assert_cmd::cargo;

#[test]
fn test_cli_basic_generation() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let output_path = dir.path().join("output.mp4");
    let audio_path = dir.path().join("input.mp3");
    let background_path = dir.path().join("background.png");
    let config_path = dir.path().join("config.yml");

    // Create a silent audio file
    Command::new("ffmpeg")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("anullsrc=r=44100:cl=mono")
        .arg("-t")
        .arg("1")
        .arg("-q:a")
        .arg("9")
        .arg("-acodec")
        .arg("libmp3lame")
        .arg(&audio_path)
        .output()?;

    // Create a 1x1 transparent PNG file
    let mut file = File::create(&background_path)?;
    file.write_all(&[
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f,
        0x15, 0xc4, 0x89, 0x00, 0x00, 0x00, 0x0a, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0d, 0x0a, 0x2d, 0xb4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ])?;

    // Create a dummy config file
    let mut file = File::create(&config_path)?;
    writeln!(file, "templates:")?;
    writeln!(file, "  default:")?;
    writeln!(file, "    video:")?;
    writeln!(file, "      width: 1920")?;
    writeln!(file, "      height: 1080")?;
    writeln!(file, "      fps: 30")?;
    writeln!(file, "    background:")?;
    writeln!(file, "      path: '{}'", background_path.to_str().unwrap())?;
    writeln!(file, "      mode: 'stretch'")?;
    writeln!(file, "    waveform:")?;
    writeln!(file, "      width: 800")?;
    writeln!(file, "      height: 300")?;
    writeln!(file, "      x: '100'")?;
    writeln!(file, "      y: '200'")?;
    writeln!(file, "      style: smooth_line")?;
    writeln!(file, "    title:")?;
    writeln!(file, "      font: 'Arial'")?;
    writeln!(file, "      size: 64")?;
    writeln!(file, "      color: 'white'")?;
    writeln!(file, "      x: '(w-text_w)/2'")?;
    writeln!(file, "      y: '540'")?;
    writeln!(file, "    subtitle:")?;
    writeln!(file, "      font: 'Arial'")?;
    writeln!(file, "      size: 32")?;
    writeln!(file, "      color: 'white'")?;
    writeln!(file, "      x: '(w-text_w)/2'")?;
    writeln!(file, "      y: '600'")?;


    let mut cmd = Command::new(cargo::cargo_bin!("audiowave"));
    cmd.arg("-i")
        .arg(&audio_path)
        .arg("-o")
        .arg(&output_path)
        .arg("-c")
        .arg(&config_path);

    cmd.assert()
        .success();

    assert!(output_path.exists());

    Ok(())
}
