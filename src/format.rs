use std::collections::BTreeMap;

use anyhow::Result;

pub fn fmt(yaml: &str) -> Result<String> {
    // Deserialize it back to a Rust type.
    let map: BTreeMap<String, f64> = serde_yaml::from_str(yaml)?;

    // Serialize it to a YAML string.
    let yaml = serde_yaml::to_string(&map)?;

    Ok(yaml)
}
