use dprint_core::plugins::FormatResult;
use ruff_python_formatter::format_module_source;

use super::configuration::Configuration;

pub fn format_text(text: &str, config: &Configuration) -> FormatResult {
    format_module_source(text, config.clone().into())
        .map_err(|err| err.into())
        .map(|result| (result.as_code() != text).then_some(result.into_code()))
}

#[cfg(test)]
mod tests {
    use super::super::configuration::resolve_config;
    use super::*;
    use dprint_core::configuration::*;

    #[test]
    fn should_error_on_syntax_diagnostic() {
        let global_config = GlobalConfiguration::default();
        let config = resolve_config(ConfigKeyMap::new(), &global_config).config;
        let message = format_text("for i range(10):", &config).err().unwrap().to_string();
        assert!(message.contains("ParseError"));
    }
}
