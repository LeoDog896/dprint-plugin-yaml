use anyhow::Result;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::generate_plugin_code;
use dprint_core::plugins::FileMatchingInfo;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::SyncPluginHandler;
use dprint_core::plugins::SyncPluginInfo;
use std::path::Path;

use crate::configuration;
use crate::configuration::config::Configuration;
use crate::format::format_text;

pub struct YamlPluginHandler {}

impl YamlPluginHandler {
    const fn new() -> Self {
        YamlPluginHandler {}
    }
}

impl SyncPluginHandler<Configuration> for YamlPluginHandler {
    fn plugin_info(&mut self) -> SyncPluginInfo {
        SyncPluginInfo {
            info: PluginInfo {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                config_key: "yaml".to_string(),
                help_url: "https://github.com/LeoDog896/dprint-plugin-yaml/issues".to_string(),
                config_schema_url: "".to_string(), // leave this empty for now
                update_url: None,                  // leave this empty for now
            },
            file_matching: FileMatchingInfo {
                file_extensions: vec![".yaml".to_string(), ".yml".to_string()],
                file_names: vec![],
            },
        }
    }

    fn license_text(&mut self) -> String {
        include_str!("../LICENSE").to_string()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        configuration::resolve::config(config, global_config)
    }

    fn format(
        &mut self,
        file_path: &Path,
        file_text: Vec<u8>,
        config: &Configuration,
        mut _format_with_host: impl FnMut(&Path, Vec<u8>, &ConfigKeyMap) -> Result<Option<Vec<u8>>>,
    ) -> Result<Option<Vec<u8>>> {
        // format here

        let yaml = format_text(file_path, String::from_utf8(file_text.as_slice().to_vec())?.as_str(), config)?;
        Ok(Some(yaml.into_bytes()))
    }
}

generate_plugin_code!(YamlPluginHandler, YamlPluginHandler::new());
