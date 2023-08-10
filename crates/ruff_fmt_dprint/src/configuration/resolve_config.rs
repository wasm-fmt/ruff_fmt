use super::{indent_style_from_global_config, line_width_from_global_config, Configuration};
use dprint_core::configuration::*;
use ruff_formatter::FormatOptions;
use ruff_python_formatter::{MagicTrailingComma, QuoteStyle};

pub fn resolve_config(
    config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
    let mut diagnostics = Vec::new();
    let mut config = config;

    let default_config = Configuration::default();

    let indent_style = get_value(
        &mut config,
        "indentStyle",
        indent_style_from_global_config(global_config).unwrap_or(default_config.indent_style()),
        &mut diagnostics,
    );
    let line_width = get_value(
        &mut config,
        "lineWidth",
        line_width_from_global_config(global_config).unwrap_or(default_config.line_width()),
        &mut diagnostics,
    );

    let quote_style = get_value(&mut config, "quoteStyle", QuoteStyle::default(), &mut diagnostics);

    let magic_trailing_comma = get_value(
        &mut config,
        "magicTrailingComma",
        MagicTrailingComma::default(),
        &mut diagnostics,
    );

    diagnostics.extend(get_unknown_property_diagnostics(config));

    let resolved_config = default_config
        .with_indent_style(indent_style)
        .with_line_width(line_width)
        .with_quote_style(quote_style)
        .with_magic_trailing_comma(magic_trailing_comma);

    ResolveConfigurationResult { config: resolved_config, diagnostics }
}
