use dprint_core::configuration::GlobalConfiguration;
pub use ruff_fmt_config::Config as Configuration;
use ruff_fmt_config::{IndentStyle, LineWidth, TabWidth};
use ruff_formatter::FormatOptions;
use ruff_python_formatter::PyFormatOptions;

pub(crate) fn indent_style_from_global_config(
    global_config: &GlobalConfiguration,
    default_ruff_config: &PyFormatOptions,
) -> IndentStyle {
    global_config
        .use_tabs
        .map(|use_tabs| if use_tabs { IndentStyle::Tab } else { IndentStyle::Space })
        .unwrap_or_else(|| default_ruff_config.indent_style().into())
}

pub(crate) fn indent_width_from_global_config(
    global_config: &GlobalConfiguration,
    default_ruff_config: &PyFormatOptions,
) -> TabWidth {
    global_config
        .indent_width
        .and_then(|indent_width| indent_width.try_into().ok())
        .unwrap_or_else(|| (default_ruff_config.indent_width() as u8).try_into().unwrap())
}

pub(crate) fn line_width_from_global_config(
    global_config: &GlobalConfiguration,
    default_ruff_config: &PyFormatOptions,
) -> LineWidth {
    global_config
        .line_width
        .map(u16::try_from)
        .and_then(Result::ok)
        .map(LineWidth::try_from)
        .and_then(Result::ok)
        .unwrap_or_else(|| default_ruff_config.line_width().into())
}
