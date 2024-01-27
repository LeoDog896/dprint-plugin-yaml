use dprint_core::configuration::get_unknown_property_diagnostics;
use dprint_core::configuration::get_value;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;

use super::config::Configuration;

pub fn config(config: ConfigKeyMap, global_config: &GlobalConfiguration) -> ResolveConfigurationResult<Configuration> {
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
