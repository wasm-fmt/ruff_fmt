/* @ts-self-types="./ruff_fmt_web.d.ts" */
import * as import_bg from "./ruff_fmt_bg.js";
const { __wbg_set_wasm, format, format_range, ...wasmImport } = import_bg;

function getImports() {
	return {
		__proto__: null,
		"./ruff_fmt_bg.js": wasmImport,
	};
}

let wasm, wasmModule;

async function load(module, imports) {
	if (typeof Response === "function" && module instanceof Response) {
		if (typeof WebAssembly.instantiateStreaming === "function") {
			try {
				return await WebAssembly.instantiateStreaming(module, imports);
			} catch (e) {
				const validResponse = module.ok && expectedResponseType(module.type);

				if (validResponse && module.headers.get("Content-Type") !== "application/wasm") {
					console.warn(
						"`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",
						e,
					);
				} else {
					throw e;
				}
			}
		}

		const bytes = await module.arrayBuffer();
		return await WebAssembly.instantiate(bytes, imports);
	} else {
		const instance = await WebAssembly.instantiate(module, imports);

		if (instance instanceof WebAssembly.Instance) {
			return { instance, module };
		} else {
			return instance;
		}
	}

	function expectedResponseType(type) {
		switch (type) {
			case "basic":
			case "cors":
			case "default":
				return true;
		}
		return false;
	}
}

function finalize_init(instance, module) {
	wasm = instance.exports;
	wasmModule = module;
	__wbg_set_wasm(wasm);
	return wasm;
}

export function initSync(module) {
	if (wasm !== void 0) return wasm;

	if (!(module instanceof WebAssembly.Module)) {
		module = new WebAssembly.Module(module);
	}
	const instance = new WebAssembly.Instance(module, getImports());
	return finalize_init(instance, module);
}

export default async function initAsync(module_or_path) {
	if (wasm !== void 0) return wasm;

	if (module_or_path === void 0) {
		module_or_path = new URL("ruff_fmt_bg.wasm", import.meta.url);
	}

	if (
		typeof module_or_path === "string" ||
		(typeof Request === "function" && module_or_path instanceof Request) ||
		(typeof URL === "function" && module_or_path instanceof URL)
	) {
		module_or_path = fetch(module_or_path);
	}

	const { instance, module } = await load(await module_or_path, getImports());

	return finalize_init(instance, module);
}

export { format, format_range };
