#[cfg(test)]
mod test;

use ruff_python_formatter::format_module;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn format(input: &str) -> Result<String, String> {
    format_module(input, Default::default())
        .map(|result| result.as_code().to_string())
        .map_err(|err| err.to_string())
}

