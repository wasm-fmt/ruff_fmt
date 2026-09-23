#!/usr/bin/env node --test
import assert from "node:assert/strict";
import { glob, readFile } from "node:fs/promises";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { parseSnapshot } from "../test_utils/index.js";

import { createConfig, format, formatRanges, releaseConfig } from "../pkg/ruff_fmt_node.js";

const project_root = fileURLToPath(import.meta.resolve("../"));
const snapshots_root = fileURLToPath(import.meta.resolve("../snapshots"));

for await (const snapshotPath of glob(`${snapshots_root}/*.snap`)) {
	const snapshotContent = await readFile(snapshotPath, "utf-8");
	const info = parseSnapshot(snapshotContent);
	if (!info) continue;

	const input_file = `${project_root}/${info.input_file}`;
	const contentPath = `${snapshotPath}.${info.extension}`;
	const [input, expected] = await Promise.all([readFile(input_file, "utf-8"), readFile(contentPath, "utf-8")]);

	test(info.input_file, () => {
		const actual = format(input, info.input_file);
		assert.equal(actual, expected);
	});
}

test("inline config", () => {
	assert.equal(format('x = "hello"\n', { quote_style: "single" }), "x = 'hello'\n");
});

test("registered config handle", () => {
	const config = createConfig({ quote_style: "single" });
	try {
		assert.equal(format('x = "hello"\n', config), "x = 'hello'\n");
	} finally {
		releaseConfig(config);
	}
});

test("null preserves the legacy optional argument semantics", () => {
	assert.equal(format("x=1", null, null), format("x=1"));
});

test("invalid JSON config is rejected during registration", () => {
	assert.throws(() => createConfig("{"), /EOF while parsing an object/);
});

test("range formatting preserves explicit empty ranges", () => {
	const source = "x=1\n";
	assert.equal(formatRanges(source, []), source);
});

test("range formatting uses UTF-8 byte offsets", () => {
	const prefix = "é = 1\n";
	const source = `${prefix}x=2\n`;
	const start = new TextEncoder().encode(prefix).length;
	const end = new TextEncoder().encode(source).length;

	assert.equal(formatRanges(source, [{ start, end }]), `${prefix}x = 2\n`);
});

test("range formatting reports unsupported multiple ranges", () => {
	const source = "x=1\ny=2\n";
	assert.throws(
		() =>
			formatRanges(source, [
				{ start: 0, end: 3 },
				{ start: 4, end: 7 },
			]),
		/exactly one range/,
	);
});
