use super::template::Template;
use directories::ProjectDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::{env, path::PathBuf, fs};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub templates: HashMap<String, Template>,
}

const DEFAULT_YAML: &str = include_str!("../../assets/default_config.yml");

impl Config {
    pub async fn load(path: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
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
            println!("✨ Configuración no encontrada. Se ha creado una por defecto en: {:?}", config_path);
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
