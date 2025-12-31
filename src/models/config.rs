use super::template::Template;
use directories::ProjectDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::{env, path::PathBuf, fs};
use log::debug;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub templates: HashMap<String, Template>,
}

const DEFAULT_YAML: &str = include_str!("../../assets/default_config.yml");

impl Config {
    pub async fn load(path: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        debug!("Cargando configuración...");
        let config_path = match path {
            Some(p) => {
                let pb = PathBuf::from(p);
                if pb.exists() {
                    pb
                } else {
                    return Err("Config file not found in provided path".into());
                }
            }
            None => {
                // Intentamos buscarlo o crearlo si no existe
                Self::get_or_create_config()?
            }
        };

        let content = tokio::fs::read_to_string(config_path).await?;
        serde_yaml::from_str(&content).map_err(|e| e.into())
    }

    /// Busca el archivo o lo crea a partir del recurso embebido si no lo encuentra
    fn get_or_create_config() -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Some(path) = Self::find_config_file() {
            return Ok(path);
        }

        // Si no existe, lo creamos en el directorio XDG del usuario
        let proj_dirs = ProjectDirs::from("es", "atareao", "audiowave")
            .ok_or("No se pudo determinar el directorio de configuración del usuario")?;
        
        let config_dir = proj_dirs.config_dir();
        let config_path = config_dir.join("config.yml");

        if !config_path.exists() {
            fs::create_dir_all(config_dir)?;
            fs::write(&config_path, DEFAULT_YAML)?;
            debug!("✨ Configuración no encontrada. Se ha creado una por defecto en: {:?}", config_path);
        }

        Ok(config_path)
    }

    fn find_config_file() -> Option<PathBuf> {
        let filename = "config.yml";

        // 1. Directorio del ejecutable
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let local_config = exe_dir.join(filename);
                if local_config.exists() {
                    return Some(local_config);
                }
            }
        }

        // 2. Directorio XDG (~/.config/audiowave/config.yml)
        if let Some(proj_dirs) = ProjectDirs::from("es", "atareao", "audiowave") {
            let user_config = proj_dirs.config_dir().join(filename);
            if user_config.exists() {
                return Some(user_config);
            }
        }

        // 3. Configuración global /etc
        let global_config = PathBuf::from("/etc/audiowave").join(filename);
        if global_config.exists() {
            return Some(global_config);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_load_from_path() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.yml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "templates:").unwrap();
        writeln!(file, "  default:").unwrap();
        writeln!(file, "    video:").unwrap();
        writeln!(file, "      width: 1920").unwrap();
        writeln!(file, "      height: 1080").unwrap();
        writeln!(file, "      fps: 30").unwrap();
        writeln!(file, "    background:").unwrap();
        writeln!(file, "      path: 'background.png'").unwrap();
        writeln!(file, "      mode: 'stretch'").unwrap();
        writeln!(file, "    waveform:").unwrap();
        writeln!(file, "      width: 800").unwrap();
        writeln!(file, "      height: 300").unwrap();
        writeln!(file, "      x: '100'").unwrap();
        writeln!(file, "      y: '200'").unwrap();
        writeln!(file, "      style: classic_line").unwrap();
        writeln!(file, "    title:").unwrap();
        writeln!(file, "      font: 'Arial'").unwrap();
        writeln!(file, "      size: 64").unwrap();
        writeln!(file, "      color: 'white'").unwrap();
        writeln!(file, "      x: '(w-text_w)/2'").unwrap();
        writeln!(file, "      y: '540'").unwrap();
        writeln!(file, "    subtitle:").unwrap();
        writeln!(file, "      font: 'Arial'").unwrap();
        writeln!(file, "      size: 32").unwrap();
        writeln!(file, "      color: 'white'").unwrap();
        writeln!(file, "      x: '(w-text_w)/2'").unwrap();
        writeln!(file, "      y: '600'").unwrap();

        let config = Config::load(Some(file_path.to_str().unwrap().to_string())).await.unwrap();
        assert!(config.templates.contains_key("default"));
        assert_eq!(config.templates.get("default").unwrap().video.width, 1920);
    }

    #[tokio::test]
    async fn test_load_not_found() {
        let result = Config::load(Some("non_existent_file.yml".to_string())).await;
        assert!(result.is_err());
    }
}
