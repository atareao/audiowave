use serde::Deserialize;
use std::collections::HashMap;
use super::template::Template;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub templates: HashMap<String, Template>,
}
