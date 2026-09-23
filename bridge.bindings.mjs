import { defineBindings } from "@wasm-fmt/bindgen";

export default defineBindings({
	name: "ruff_fmt",
	wasm: "target/wasm32-unknown-unknown/release/ruff_fmt.wasm",
	wasmFile: "ruff_fmt_bg.wasm",
	adapter: "bindings/ruff_fmt_binding.js",
	types: {
		main: "bindings/ruff_fmt.d.ts",
	},
	assets: [
		"package.json",
		"jsr.jsonc",
		"README.md",
		"LICENSE-MIT",
		"LICENSE-APACHE",
		"bindings/.npmignore",
		"bindings/ruff_fmt_config.d.ts",
	],
	outDir: "pkg",
	clean: true,
});
