use dprint_core::configuration::GlobalConfiguration;
use ruff_formatter::{IndentStyle, LineWidth};
pub use ruff_python_formatter::PyFormatOptions as Configuration;

pub(crate) fn indent_style_from_global_config(
    global_config: &GlobalConfiguration,
) -> Option<IndentStyle> {
    let use_tab: Option<bool> = global_config.use_tabs;
    let indent_width = global_config.indent_width;

    match (use_tab, indent_width) {
        (Some(true), _) => Some(IndentStyle::Tab),
        (_, Some(width)) => Some(IndentStyle::Space(width)),
        _ => None,
    }
}

pub(crate) fn line_width_from_global_config(
    global_config: &GlobalConfiguration,
) -> Option<LineWidth> {
    let Some(line_width) = global_config.line_width else {
        return None;
    };

    u16::try_from(line_width).ok().map(LineWidth::try_from).and_then(Result::ok)
}
