use std::{collections::BTreeMap, path::Path};

use anyhow::Result;

use crate::configuration::config::Configuration;

pub fn format_text(_file_path: &Path, yaml: &str, _config: &Configuration) -> Result<String> {
    // Deserialize it back to a Rust type.
    let map: BTreeMap<String, f64> = serde_yaml::from_str(yaml)?;

    // Serialize it to a YAML string.
    let yaml = serde_yaml::to_string(&map)?;

    Ok(yaml)
}
