#[cfg(test)]
mod test;

pub mod config;

use config::Config as InnerConfig;
use ruff_python_formatter::format_module_source;
use ruff_python_formatter::format_range as ruff_format_range;
use ruff_text_size::TextRange as RuffTextRange;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_Types: &'static str = r#"
/**
 * A range in text, using UTF-8 byte offsets.
 */
export interface TextRange {
	start: number;
	end: number;
}

/**
 * Result of formatting a range of code.
 */
export interface PrintedRange {
	code: string;
	source_range: TextRange;
}"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Config")]
    pub type JsConfig;

    #[wasm_bindgen(typescript_type = "TextRange")]
    pub type JsTextRange;

    #[wasm_bindgen(typescript_type = "PrintedRange")]
    pub type JsPrintedRange;
}

/// Format the entire Python source code string.
#[wasm_bindgen]
pub fn format(
    input: &str,
    path: Option<String>,
    config: Option<JsConfig>,
) -> Result<String, String> {
    let mut config: InnerConfig = config
        .map(|c| serde_wasm_bindgen::from_value(c.into()).map_err(|e| e.to_string()))
        .transpose()?
        .unwrap_or_default();

    if let Some(path) = path {
        config = config.with_path(path);
    }

    format_module_source(input, config.into())
        .map(|result| result.into_code())
        .map_err(|err| err.to_string())
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct TextRange {
    pub start: u32,
    pub end: u32,
}

impl From<RuffTextRange> for TextRange {
    fn from(range: RuffTextRange) -> Self {
        Self { start: range.start().into(), end: range.end().into() }
    }
}

impl From<TextRange> for RuffTextRange {
    fn from(range: TextRange) -> Self {
        Self::new(range.start.into(), range.end.into())
    }
}

#[derive(Serialize, Deserialize)]
pub struct PrintedRange {
    pub code: String,
    pub source_range: TextRange,
}

impl From<ruff_formatter::PrintedRange> for PrintedRange {
    fn from(range: ruff_formatter::PrintedRange) -> Self {
        Self { source_range: range.source_range().into(), code: range.into_code() }
    }
}

/// Format a specific range of the Python source code string.
#[wasm_bindgen(unchecked_return_type = "PrintedRange")]
pub fn format_range(
    input: &str,
    range: JsTextRange,
    path: Option<String>,
    config: Option<JsConfig>,
) -> Result<JsValue, JsValue> {
    let text_range: TextRange = serde_wasm_bindgen::from_value(range.into())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mut config: InnerConfig = config
        .map(|c| serde_wasm_bindgen::from_value(c.into()).map_err(|e| e.to_string()))
        .transpose()?
        .unwrap_or_default();

    if let Some(path) = path {
        config = config.with_path(path);
    }

    let range = text_range.into();
    let config = config.into();

    let result =
        ruff_format_range(input, range, config).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let printed_range = PrintedRange::from(result);

    serde_wasm_bindgen::to_value(&printed_range).map_err(|e| JsValue::from_str(&e.to_string()))
}
