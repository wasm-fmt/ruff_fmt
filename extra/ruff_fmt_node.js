import fs from "node:fs/promises";
import initAsync from "./ruff_fmt.js";

const wasm = new URL("./ruff_fmt_bg.wasm", import.meta.url);

export default function __wbg_init(init = { module_or_path: fs.readFile(wasm) }) {
    return initAsync(init);
}

export * from "./ruff_fmt.js";
