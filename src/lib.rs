use anyhow::Result;
use dprint_core::configuration::get_unknown_property_diagnostics;
use dprint_core::configuration::get_value;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::plugins::FileMatchingInfo;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::SyncPluginHandler;
use dprint_core::plugins::SyncPluginInfo;
use dprint_core::generate_plugin_code;
use format::fmt;
use serde::Serialize;
use std::path::Path;

mod format;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    // add configuration properties here...
    line_width: u32, // for example
}

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
                file_names: vec![]
            }
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
        // implement this... for example
        let mut config = config;
        let mut diagnostics = Vec::new();
        let line_width = get_value(
            &mut config,
            "line_width",
            global_config.line_width.unwrap_or(120),
            &mut diagnostics,
        );

        diagnostics.extend(get_unknown_property_diagnostics(config));

        ResolveConfigurationResult {
            config: Configuration { line_width },
            diagnostics,
        }
    }
    
    fn format(
        &mut self,
        _file_path: &Path,
        file_text: Vec<u8>,
        _config: &Configuration,
        mut _format_with_host: impl FnMut(&Path, Vec<u8>, &ConfigKeyMap) -> Result<Option<Vec<u8>>>,
    ) -> Result<Option<Vec<u8>>> {
        // format here

        let yaml = fmt(String::from_utf8(file_text.as_slice().to_vec())?.as_str())?;
        Ok(Some(yaml.into_bytes()))
    }
}

generate_plugin_code!(YamlPluginHandler, YamlPluginHandler::new());
