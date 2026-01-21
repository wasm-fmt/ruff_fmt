#!/usr/bin/env node

import fs from "fs";
import { fileURLToPath } from "url";

const version = process.argv[2];

const package_json_path = fileURLToPath(import.meta.resolve("../package.json"));
const jsr_path = fileURLToPath(import.meta.resolve("../jsr.jsonc"));

const pack_json = readJSON(package_json_path);
const jsr = readJSON(jsr_path);

pack_json.version = version;
jsr.version = version;

writeJSON(package_json_path, pack_json);
writeJSON(jsr_path, jsr);

function readJSON(path) {
	const content = fs.readFileSync(path, "utf-8");
	return JSON.parse(content);
}

function writeJSON(path, data) {
	const content = JSON.stringify(data, null, "\t") + "\n";
	fs.writeFileSync(path, content, "utf-8");
}
