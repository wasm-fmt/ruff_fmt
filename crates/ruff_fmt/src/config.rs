use std::borrow::Cow;

use ruff_formatter::{IndentStyle as RuffIndentStyle, LineWidth};
use ruff_python_formatter::{MagicTrailingComma, PyFormatOptions, QuoteStyle};
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface Config {
    indent_style?: "tab" | number;
    line_width?: number;
    quote_style?: "single" | "double";
    magic_trailing_comma?: "respect" | "ignore";
}"#;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Config {
    indent_style: Option<IndentStyle>,
    line_width: Option<u16>,
    quote_style: Option<QuoteStyle>,
    magic_trailing_comma: Option<MagicTrailingComma>,
}

impl From<Config> for PyFormatOptions {
    fn from(value: Config) -> Self {
        let mut default = Self::default();

        if let Some(indent_style) = value.indent_style {
            default = default.with_indent_style(indent_style.into());
        }

        if let Some(line_width) = value.line_width.map(LineWidth::try_from).and_then(Result::ok) {
            default = default.with_line_width(line_width);
        }

        if let Some(quote_style) = value.quote_style {
            default = default.with_quote_style(quote_style);
        }

        if let Some(magic_trailing_comma) = value.magic_trailing_comma {
            default = default.with_magic_trailing_comma(magic_trailing_comma);
        }

        default
    }
}

pub(crate) enum IndentStyle {
    Tab,
    Space(u8),
}

impl<'de> Deserialize<'de> for IndentStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum IndentStyle<'a> {
            Tab(Cow<'a, str>),
            Space(u8),
        }

        match IndentStyle::deserialize(deserializer)? {
            IndentStyle::Space(n) => Ok(Self::Space(n)),
            IndentStyle::Tab(c) if c == "tab" => Ok(Self::Tab),
            IndentStyle::Tab(other) => {
                Err(D::Error::invalid_value(Unexpected::Str(&other), &"<number> or `tab`"))
            }
        }
    }
}

impl From<IndentStyle> for RuffIndentStyle {
    fn from(value: IndentStyle) -> Self {
        match value {
            IndentStyle::Tab => Self::Tab,
            IndentStyle::Space(count) => Self::Space(count),
        }
    }
}
