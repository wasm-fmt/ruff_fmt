#!/usr/bin/env node
import process from "node:process";
import path from "node:path";
import fs from "node:fs";

const pkg_path = path.resolve(process.cwd(), process.argv[2]);
const pkg_text = fs.readFileSync(pkg_path, { encoding: "utf-8" });
const pkg_json = JSON.parse(pkg_text);

delete pkg_json.files;

pkg_json.main = pkg_json.module;
pkg_json.type = "module";
pkg_json.publishConfig = {
	access: "public",
};
pkg_json.exports = {
	// Deno
	// - 2.6 supports wasm source phase imports
	// - 2.1 support wasm instance phase imports
	// Node.js
	// - 24.5.0 unflag source phase imports for webassembly
	// - 24.0.0 supports source phase imports for webassembly
	// - 22.19.0 backport source/instance phase imports for webassembly
	".": {
		types: "./ruff_fmt.d.ts",
		webpack: "./ruff_fmt.js",
		deno: "./ruff_fmt.js",
		// CJS supports
		"module-sync": "./ruff_fmt_node.js",
		default: "./ruff_fmt_esm.js",
	},
	"./esm": {
		types: "./ruff_fmt.d.ts",
		default: "./ruff_fmt_esm.js",
	},
	"./node": {
		types: "./ruff_fmt.d.ts",
		default: "./ruff_fmt_node.js",
	},
	"./bundler": {
		types: "./ruff_fmt.d.ts",
		default: "./ruff_fmt.js",
	},
	"./web": {
		types: "./ruff_fmt_web.d.ts",
		default: "./ruff_fmt_web.js",
	},
	"./vite": {
		types: "./ruff_fmt_web.d.ts",
		default: "./ruff_fmt_vite.js",
	},
	"./wasm": "./ruff_fmt_bg.wasm",
	"./package.json": "./package.json",
	"./*": "./*",
};
pkg_json.sideEffects = ["./ruff_fmt.js", "./ruff_fmt_node.js", "./ruff_fmt_esm.js"];

fs.writeFileSync(pkg_path, JSON.stringify(pkg_json, null, "\t"));

// JSR
const jsr_path = path.resolve(pkg_path, "..", "jsr.jsonc");
pkg_json.name = "@fmt/ruff-fmt";
pkg_json.exports = {
	".": "./ruff_fmt.js",
	"./esm": "./ruff_fmt_esm.js",
	"./node": "./ruff_fmt_node.js",
	"./bundler": "./ruff_fmt.js",
	"./web": "./ruff_fmt_web.js",
	// jsr does not support imports from wasm?init
	// "./vite": "./ruff_fmt_vite.js",
};
pkg_json.exclude = ["!**", "*.tgz"];
fs.writeFileSync(jsr_path, JSON.stringify(pkg_json, null, "\t"));

const ruff_fmt_path = path.resolve(path.dirname(pkg_path), "ruff_fmt.js");
prependTextToFile('/* @ts-self-types="./ruff_fmt.d.ts" */\n', ruff_fmt_path);

const ruff_fmt_d_ts_path = path.resolve(path.dirname(pkg_path), "ruff_fmt.d.ts");
const doc_path = path.resolve(import.meta.dirname, "doc.d.ts");
const doc_text = fs.readFileSync(doc_path, { encoding: "utf-8" });
prependTextToFile(doc_text + "\n", ruff_fmt_d_ts_path);

function prependTextToFile(text, filePath) {
	const originalContent = fs.readFileSync(filePath, { encoding: "utf-8" });
	const newContent = text + originalContent;
	fs.writeFileSync(filePath, newContent);
}
