use id3::{Tag, TagLike};
use std::path::PathBuf;
use tokio::task;

pub struct AudioMetadata {
    pub title: String,
    pub artist: String,
    pub cover_path: Option<PathBuf>,
}

impl AudioMetadata {
    pub async fn new(input_path: String) -> Self {
        task::spawn_blocking(move || {
            let mut meta = AudioMetadata {
                title: "Nuevo Episodio".to_string(),
                artist: "atareao con Linux".to_string(),
                cover_path: None,
            };

            if let Ok(tag) = Tag::read_from_path(&input_path) {
                if let Some(t) = tag.title() {
                    meta.title = t.to_string();
                }
                if let Some(a) = tag.artist() {
                    meta.artist = a.to_string();
                }

                // Extraer carátula si existe (APIC tag)
                if let Some(pic) = tag.pictures().next() {
                    // Creamos un archivo temporal para la imagen
                    if let Ok(temp) = tempfile::Builder::new()
                        .prefix("audiowave_cover_")
                        .suffix(".jpg")
                        .tempfile()
                    {
                        let (mut file, path) =
                            temp.keep().expect("Error al guardar imagen temporal");
                        if std::io::Write::write_all(&mut file, &pic.data).is_ok() {
                            meta.cover_path = Some(path);
                        }
                    }
                }
            }
            meta
        })
        .await
        .unwrap_or_else(|_| AudioMetadata {
            title: "Error".to_string(),
            artist: "Metadata".to_string(),
            cover_path: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_with_non_existent_file() {
        let metadata = AudioMetadata::new("non_existent_file.mp3".to_string()).await;
        assert_eq!(metadata.title, "Nuevo Episodio");
        assert_eq!(metadata.artist, "atareao con Linux");
        assert!(metadata.cover_path.is_none());
    }
}
