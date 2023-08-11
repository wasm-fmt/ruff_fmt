use std::path::Path;

use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::generate_plugin_code;
use dprint_core::plugins::FileMatchingInfo;
use dprint_core::plugins::FormatResult;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::SyncPluginHandler;
use dprint_core::plugins::SyncPluginInfo;
use serde::Deserialize;
use serde::Serialize;

use crate::format_text;

use super::configuration::resolve_config;
use super::configuration::Configuration;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RuffFmtWasmPlugin;

impl RuffFmtWasmPlugin {
    pub const fn new() -> Self {
        RuffFmtWasmPlugin {}
    }
}

impl SyncPluginHandler<Configuration> for RuffFmtWasmPlugin {
    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        resolve_config(config, global_config)
    }

    fn plugin_info(&mut self) -> SyncPluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_string();
        let config_schema_url =
            format!("https://plugins.dprint.dev/wasm-fmt/ruff_fmt/{}/schema.json", version);

        SyncPluginInfo {
            info: PluginInfo {
                name: "ruff_fmt".to_string(),
                version,
                config_key: "python".to_string(),
                help_url: format!("{}/issues", env!("CARGO_PKG_REPOSITORY")),
                config_schema_url,
                update_url: Some(
                    "https://plugins.dprint.dev/wasm-fmt/ruff_fmt/latest.json".to_string(),
                ),
            },
            file_matching: FileMatchingInfo {
                file_extensions: vec!["py".to_string()],
                file_names: vec![],
            },
        }
    }

    fn license_text(&mut self) -> String {
        std::str::from_utf8(include_bytes!("../../../LICENSE")).unwrap().into()
    }

    fn format(
        &mut self,
        _file_path: &std::path::Path,
        file_text: &str,
        config: &Configuration,
        mut _format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> FormatResult,
    ) -> FormatResult {
        format_text(file_text, config)
    }
}

generate_plugin_code!(RuffFmtWasmPlugin, RuffFmtWasmPlugin::new());
