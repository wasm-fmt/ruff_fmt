import initAsync from "./ruff_fmt.js";
import wasm from "./ruff_fmt_bg.wasm?url";

export default function __wbg_init(input = { module_or_path: wasm }) {
    return initAsync(input);
}

export * from "./ruff_fmt.js";
