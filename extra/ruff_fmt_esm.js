/* @ts-self-types="./ruff_fmt.d.ts" */
// prettier-ignore
import source wasmModule from "./ruff_fmt_bg.wasm";

import * as import_bg from "./ruff_fmt_bg.js";
const { __wbg_set_wasm, format, format_range, ...wasmImport } = import_bg;

function getImports() {
	return {
		__proto__: null,
		"./ruff_fmt_bg.js": wasmImport,
	};
}

const instance = new WebAssembly.Instance(wasmModule, getImports());

/**
 * @import * as WASM from "./ruff_fmt_bg.wasm"
 */

/**
 * @type {WASM}
 */
const wasm = instance.exports;
__wbg_set_wasm(wasm);

export { format, format_range };
