#!/usr/bin/env deno test --allow-read --parallel
import { assertEquals } from "jsr:@std/assert";
import { expandGlob } from "jsr:@std/fs";
import { fromFileUrl } from "jsr:@std/path";
import { parseSnapshot } from "../test_utils/index.js";

import { format } from "../pkg/ruff_fmt_esm.js";

const project_root = fromFileUrl(import.meta.resolve("../"));
const snapshots_root = fromFileUrl(import.meta.resolve("../snapshots"));

for await (const { path: snapshotPath } of expandGlob("**/*.snap", { root: snapshots_root })) {
	const snapshotContent = await Deno.readTextFile(snapshotPath);
	const info = parseSnapshot(snapshotContent);
	if (!info) continue;

	const input_file = `${project_root}/${info.input_file}`;
	const contentPath = `${snapshotPath}.${info.extension}`;
	const [input, expected] = await Promise.all([Deno.readTextFile(input_file), Deno.readTextFile(contentPath)]);

	Deno.test(info.input_file, () => {
		const actual = format(input, info.input_file);
		assertEquals(actual, expected);
	});
}
