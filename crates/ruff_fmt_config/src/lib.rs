use std::{
    num::{NonZeroU16, NonZeroU8},
    path::Path,
    str::FromStr,
};

use ruff_formatter::{printer::LineEnding as RuffLineEnding, IndentStyle as RuffIndentStyle};
use ruff_python_formatter::{MagicTrailingComma, PyFormatOptions, QuoteStyle};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IndentStyle {
    Tab,
    Space,
}

impl FromStr for IndentStyle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tab" => Ok(Self::Tab),
            "space" => Ok(Self::Space),
            _ => Err("Value not supported for IndentStyle"),
        }
    }
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
#[serde(rename_all = "snake_case")]
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

impl FromStr for LineEnding {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lf" => Ok(Self::Lf),
            "crlf" => Ok(Self::CrLf),
            _ => Err("Value not supported for LineEnding"),
        }
    }
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(alias = "indentStyle")]
    pub indent_style: Option<IndentStyle>,
    #[serde(alias = "indentWidth")]
    pub indent_width: Option<NonZeroU8>,
    #[serde(alias = "lineWidth")]
    pub line_width: Option<NonZeroU16>,
    #[serde(alias = "lineEnding")]
    pub line_ending: Option<LineEnding>,

    pub quote_style: Option<QuoteStyle>,
    pub magic_trailing_comma: Option<MagicTrailingComma>,

    #[serde(skip)]
    path: String,
}

impl Config {
    pub fn with_indent_style(mut self, indent_style: IndentStyle) -> Self {
        self.indent_style = Some(indent_style);
        self
    }

    pub fn with_indent_width(mut self, indent_width: NonZeroU8) -> Self {
        self.indent_width = Some(indent_width);
        self
    }

    pub fn with_line_width(mut self, line_width: NonZeroU16) -> Self {
        self.line_width = Some(line_width);
        self
    }

    pub fn with_line_ending(mut self, line_ending: LineEnding) -> Self {
        self.line_ending = Some(line_ending);
        self
    }

    pub fn with_quote_style(mut self, quote_style: QuoteStyle) -> Self {
        self.quote_style = Some(quote_style);
        self
    }

    pub fn with_magic_trailing_comma(mut self, magic_trailing_comma: MagicTrailingComma) -> Self {
        self.magic_trailing_comma = Some(magic_trailing_comma);
        self
    }

    pub fn with_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }
}

impl From<Config> for PyFormatOptions {
    fn from(value: Config) -> Self {
        let mut config = Self::from_extension(Path::new(&value.path));

        if let Some(indent_style) = value.indent_style {
            config = config.with_indent_style(indent_style.into());
        }

        if let Some(indent_width) = value.indent_width {
            config = config.with_indent_width(indent_width.into());
        }

        if let Some(line_width) = value.line_width {
            config = config.with_line_width(line_width.into());
        }

        if let Some(line_ending) = value.line_ending {
            config = config.with_line_ending(line_ending.into());
        }

        if let Some(quote_style) = value.quote_style {
            config = config.with_quote_style(quote_style);
        }

        if let Some(magic_trailing_comma) = value.magic_trailing_comma {
            config = config.with_magic_trailing_comma(magic_trailing_comma);
        }

        config
    }
}
