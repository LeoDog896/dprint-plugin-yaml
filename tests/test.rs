extern crate dprint_development;
extern crate dprint_plugin_yaml;

//#[macro_use] extern crate debug_here;

use std::path::PathBuf;
// use std::time::Instant;

use dprint_core::configuration::*;
use dprint_development::*;
use dprint_plugin_yaml::{configuration::resolve, format};

#[test]
fn test_specs() {
  //debug_here!();
  let global_config = GlobalConfiguration::default();

  run_specs(
    &PathBuf::from("./tests/specs"),
    &ParseSpecOptions {
      default_file_name: "file.yml",
    },
    &RunSpecsOptions {
      fix_failures: false,
      format_twice: true,
    },
    {
      let global_config = global_config.clone();
      move |file_path, file_text, spec_config| {
        let spec_config: ConfigKeyMap = serde_json::from_value(spec_config.clone().into()).unwrap();
        let config_result = resolve::config(spec_config, &global_config);
        ensure_no_diagnostics(&config_result.diagnostics);

        format::format_text(file_path, &file_text, &config_result.config).map(|str| Some(str))
      }
    },
    move |_file_path, _file_text, _spec_config| {
      panic!("\n====\nTracing is not implemented yet\n====\n")
    },
  )
}