use std::{
    num::{NonZeroU16, NonZeroU8, ParseIntError, TryFromIntError},
    str::FromStr,
};

use ruff_formatter::{
    IndentStyle as RuffIndentStyle, IndentWidth as RuffIndentWidth, LineWidth as RuffLineWidth,
};
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

#[derive(Clone, Deserialize, Serialize)]
pub struct IndentWidth(NonZeroU8);

impl FromStr for IndentWidth {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NonZeroU8::from_str(s).map(Self)
    }
}

impl From<IndentWidth> for RuffIndentWidth {
    fn from(value: IndentWidth) -> Self {
        Self::try_from(value.0.get()).unwrap()
    }
}

impl From<RuffIndentWidth> for IndentWidth {
    fn from(value: RuffIndentWidth) -> Self {
        Self::try_from(value.value() as u8).unwrap()
    }
}

impl TryFrom<u8> for IndentWidth {
    type Error = TryFromIntError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        NonZeroU8::try_from(value).map(Self)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LineWidth(NonZeroU16);

impl FromStr for LineWidth {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NonZeroU16::from_str(s).map(Self)
    }
}

impl From<LineWidth> for RuffLineWidth {
    fn from(value: LineWidth) -> Self {
        Self::try_from(value.0.get()).unwrap()
    }
}

impl From<RuffLineWidth> for LineWidth {
    fn from(value: RuffLineWidth) -> Self {
        Self(NonZeroU16::try_from(value.value()).unwrap())
    }
}

impl TryFrom<u16> for LineWidth {
    type Error = TryFromIntError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        NonZeroU16::try_from(value).map(Self)
    }
}

#[derive(Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub indent_style: Option<IndentStyle>,
    pub indent_width: Option<IndentWidth>,
    pub line_width: Option<LineWidth>,
    pub quote_style: Option<QuoteStyle>,
    pub magic_trailing_comma: Option<MagicTrailingComma>,
}

impl Config {
    pub fn with_indent_style(mut self, indent_style: IndentStyle) -> Self {
        self.indent_style = Some(indent_style);
        self
    }

    pub fn with_indent_width(mut self, indent_width: IndentWidth) -> Self {
        self.indent_width = Some(indent_width);
        self
    }

    pub fn with_line_width(mut self, line_width: LineWidth) -> Self {
        self.line_width = Some(line_width);
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
}

impl From<Config> for PyFormatOptions {
    fn from(value: Config) -> Self {
        let mut config = Self::default();

        if let Some(indent_style) = value.indent_style {
            config = config.with_indent_style(indent_style.into());
        }

        if let Some(indent_width) = value.indent_width {
            config = config.with_indent_width(indent_width.into());
        }

        if let Some(line_width) = value.line_width {
            config = config.with_line_width(line_width.into());
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
