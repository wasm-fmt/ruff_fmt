#!/usr/bin/env node
import fs from "node:fs";
import { fileURLToPath } from "node:url";

const ruff_fmt_d_ts_path = fileURLToPath(import.meta.resolve("../pkg/ruff_fmt.d.ts"));
const doc_text = fs.readFileSync(fileURLToPath(import.meta.resolve("./doc.d.ts")), { encoding: "utf-8" });
prependTextToFile(doc_text + "\n", ruff_fmt_d_ts_path);

function prependTextToFile(text, filePath) {
	const originalContent = fs.readFileSync(filePath, { encoding: "utf-8" });
	const newContent = text + originalContent;
	fs.writeFileSync(filePath, newContent);
}
