mod config;
#[cfg(test)]
mod test;

use config::Config as InnerConfig;
use ruff_python_formatter::format_module;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn format(input: &str) -> Result<String, String> {
    format_module(input, Default::default())
        .map(|result| result.into_code())
        .map_err(|err| err.to_string())
}

#[wasm_bindgen]
pub fn format_with_config(input: &str, config: &Config) -> Result<String, String> {
    let config: &JsValue = config.as_ref();
    let config: InnerConfig =
        serde_wasm_bindgen::from_value(config.clone()).map_err(|op| op.to_string())?;

    format_module(input, config.into())
        .map(|result| result.into_code())
        .map_err(|err| err.to_string())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Config")]
    pub type Config;
}
