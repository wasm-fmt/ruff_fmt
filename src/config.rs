use std::{
    num::{NonZeroU16, NonZeroU8},
    path::Path,
};

use ruff_formatter::{
    printer::{LineEnding as RuffLineEnding, SourceMapGeneration},
    IndentStyle as RuffIndentStyle,
};
use ruff_python_formatter::{
    DocstringCode, DocstringCodeLineWidth, MagicTrailingComma, PreviewMode, PyFormatOptions,
    QuoteStyle,
};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IndentStyle {
    Tab,
    Space,
}

impl From<RuffIndentStyle> for IndentStyle {
    fn from(value: RuffIndentStyle) -> Self {
        match value {
            RuffIndentStyle::Tab => Self::Tab,
            RuffIndentStyle::Space => Self::Space,
        }
    }
}

impl From<IndentStyle> for RuffIndentStyle {
    fn from(value: IndentStyle) -> Self {
        match value {
            IndentStyle::Tab => Self::Tab,
            IndentStyle::Space => Self::Space,
        }
    }
}

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LineEnding {
    #[default]
    Lf,
    CrLf,
}

impl From<LineEnding> for RuffLineEnding {
    fn from(value: LineEnding) -> Self {
        match value {
            LineEnding::Lf => Self::LineFeed,
            LineEnding::CrLf => Self::CarriageReturnLineFeed,
        }
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
interface LayoutConfig {
    /** Specifies the indent style: Either a tab or a specific amount of spaces */
    indent_style?: "tab" | "space";
    /** The visual width of a tab character */
    indent_width?: number;
    /** The preferred line width at which the formatter should wrap lines */
    line_width?: number;
    /** The type of line ending to apply to the printed input */
    line_ending?: "lf" | "crlf";
}

/** Configuration for the Python formatter */
export interface Config extends LayoutConfig {
    /** The preferred quote style to use (single vs double quotes) */
    quote_style?: "single" | "double" | "preserve";
    /** Whether to expand lists or elements if they have a trailing comma */
    magic_trailing_comma?: "respect" | "ignore";
    /** Whether to format code snippets in docstrings or not */
    docstring_code?: boolean;
    /** The preferred line width at which the formatter should wrap lines in docstring code examples */
    docstring_code_line_width?: number | "dynamic";
    /** Whether the formatter should generate a source map */
    source_map_generation?: boolean;
    /** Whether preview style formatting is enabled or not */
    preview?: boolean;
}"#;

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct LayoutConfig {
    /// Specifies the indent style:
    /// * Either a tab
    /// * or a specific amount of spaces
    #[serde(alias = "indentStyle")]
    pub indent_style: Option<IndentStyle>,

    /// The visual width of a tab character.
    #[serde(alias = "indentWidth")]
    pub indent_width: Option<NonZeroU8>,

    /// The preferred line width at which the formatter should wrap lines.
    #[serde(alias = "lineWidth")]
    pub line_width: Option<NonZeroU16>,

    /// The type of line ending to apply to the printed input.
    #[serde(alias = "lineEnding")]
    pub line_ending: Option<LineEnding>,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct LanguageOptions {
    /// The (minimum) Python version used to run the formatted code. This is used
    /// to determine the supported Python syntax.
    target_version: Option<String>,

    /// The preferred quote style to use (single vs double quotes).
    quote_style: Option<QuoteStyle>,

    /// Whether to expand lists or elements if they have a trailing comma such as `(a, b,)`.
    magic_trailing_comma: Option<MagicTrailingComma>,

    /// Should the formatter generate a source map that allows mapping source positions to positions
    /// in the formatted document.
    source_map_generation: Option<SourceMapGeneration>,

    /// Whether to format code snippets in docstrings or not.
    ///
    /// By default this is disabled (opt-in), but the plan is to make this
    /// enabled by default (opt-out) in the future.
    docstring_code: Option<DocstringCode>,

    /// The preferred line width at which the formatter should wrap lines in
    /// docstring code examples. This only has an impact when `docstring_code`
    /// is enabled.
    docstring_code_line_width: Option<DocstringCodeLineWidth>,

    /// Whether preview style formatting is enabled or not
    preview: Option<PreviewMode>,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(flatten)]
    pub layout: LayoutConfig,

    #[serde(flatten)]
    pub language: LanguageOptions,

    #[serde(skip)]
    path: String,
}

impl Config {
    pub fn with_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }
}

impl From<Config> for PyFormatOptions {
    fn from(value: Config) -> Self {
        let mut config = Self::from_extension(Path::new(&value.path));

        if let Some(indent_style) = value.layout.indent_style {
            config = config.with_indent_style(indent_style.into());
        }
        if let Some(indent_width) = value.layout.indent_width {
            config = config.with_indent_width(indent_width.into());
        }
        if let Some(line_width) = value.layout.line_width {
            config = config.with_line_width(line_width.into());
        }
        if let Some(line_ending) = value.layout.line_ending {
            config = config.with_line_ending(line_ending.into());
        }
        if let Some(target_version) = value.language.target_version {
            if let Ok(version) = target_version.parse() {
                config = config.with_target_version(version);
            }
        }
        if let Some(quote_style) = value.language.quote_style {
            config = config.with_quote_style(quote_style);
        }
        if let Some(magic_trailing_comma) = value.language.magic_trailing_comma {
            config = config.with_magic_trailing_comma(magic_trailing_comma);
        }
        if let Some(source_map_generation) = value.language.source_map_generation {
            config = config.with_source_map_generation(source_map_generation);
        }
        if let Some(docstring_code) = value.language.docstring_code {
            config = config.with_docstring_code(docstring_code);
        }
        if let Some(docstring_code_line_width) = value.language.docstring_code_line_width {
            config = config.with_docstring_code_line_width(docstring_code_line_width);
        }
        if let Some(preview) = value.language.preview {
            config = config.with_preview(preview);
        }

        config
    }
}
