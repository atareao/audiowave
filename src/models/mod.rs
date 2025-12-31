pub mod video;
pub mod background;
pub mod waveform;
pub mod text;
pub mod template;
pub mod config;
pub mod style;

// Re-exportar para facilitar el uso
pub use background::BackgroundSettings;
pub use config::Config;
pub use template::Template;
pub use text::TextSettings;
pub use video::VideoSettings;
pub use waveform::WaveformSettings;
