#[cfg(test)]
mod test;

pub mod config;

use std::ops::Range;

use bridge::{FormatResult, TextEdit};
use config::Config as InnerConfig;
use ruff_python_formatter::{format_module_source, format_range as format_python_range};
use ruff_text_size::{TextRange, TextSize};

/// Format the entire Python source code string.
#[bridge::formatter]
fn format(source: &str, filename: Option<&str>, config: &InnerConfig) -> Result<String, String> {
    format_module_source(source, format_options(config, filename))
        .map(|result| result.into_code())
        .map_err(|err| err.to_string())
}

/// Format one byte range and return Ruff's actual replacement range.
#[bridge::formatter]
fn format_range(
    source: &str,
    ranges: &[Range<u32>],
    filename: Option<&str>,
    config: &InnerConfig,
) -> Result<FormatResult, String> {
    let [range] = ranges else {
        if ranges.is_empty() {
            return Ok(FormatResult::Unchanged);
        }
        return Err("Ruff range formatting currently accepts exactly one range".to_string());
    };

    let requested_range = TextRange::new(TextSize::new(range.start), TextSize::new(range.end));
    let printed = format_python_range(source, requested_range, format_options(config, filename))
        .map_err(|err| err.to_string())?;
    let source_range = printed.source_range();
    let text = printed.into_code();

    if source_range.is_empty() && text.is_empty() {
        return Ok(FormatResult::Unchanged);
    }

    Ok(FormatResult::PartialUpdate(vec![TextEdit {
        range: u32::from(source_range.start())..u32::from(source_range.end()),
        text,
    }]))
}

fn format_options(
    config: &InnerConfig,
    filename: Option<&str>,
) -> ruff_python_formatter::PyFormatOptions {
    let mut config = config.clone();

    if let Some(filename) = filename {
        config = config.with_path(filename.to_owned());
    }

    config.into()
}
