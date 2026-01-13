#!/usr/bin/env bun test
import { Glob } from "bun";
import { expect, test } from "bun:test";
import { parseSnapshot } from "../test_utils";

import init, { format } from "../pkg/ruff_fmt_web.js";

await init();

const project_root = Bun.fileURLToPath(import.meta.resolve("../"));
const snapshots_root = Bun.fileURLToPath(import.meta.resolve("../snapshots"));

for await (const snap_path of new Glob("*.snap").scan({ cwd: snapshots_root })) {
	// Only process metadata files (not the .snap.py content files)
	const snapshotPath = `${snapshots_root}/${snap_path}`;
	const snapshotContent = await Bun.file(snapshotPath).text();
	const info = parseSnapshot(snapshotContent);
	if (!info) continue;

	const input_file = Bun.file(`${project_root}/${info.input_file}`);
	const expected_file = Bun.file(`${snapshotPath}.${info.extension}`);

	const [input, expected] = await Promise.all([input_file.text(), expected_file.text()]);

	test(info.input_file, () => {
		const actual = format(input, info.input_file);
		expect(actual).toBe(expected);
	});
}
