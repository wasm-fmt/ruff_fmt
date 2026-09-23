interface LayoutConfig {
	indent_style?: "tab" | "space";
	indent_width?: number;
	line_width?: number;
	line_ending?: "lf" | "crlf";
}

/** Configuration for the Python formatter. */
export interface Config extends LayoutConfig {
	quote_style?: "single" | "double" | "preserve";
	magic_trailing_comma?: "respect" | "ignore";
	docstring_code?: boolean;
	docstring_code_line_width?: number | "dynamic";
	source_map_generation?: boolean;
	preview?: boolean;
	nested_string_quote_style?: "alternating" | "preferred";
}
