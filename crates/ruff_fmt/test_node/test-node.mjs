import assert from "node:assert/strict";
import { glob, readFile } from "node:fs/promises";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { parseSnapshot } from "../test_utils/index.js";

import init, { format } from "../pkg/ruff_fmt_node.js";

await init();

const project_root = fileURLToPath(import.meta.resolve("../../.."));
const snapshots_root = fileURLToPath(import.meta.resolve("../snapshots"));

for await (const snapshotPath of glob(`${snapshots_root}/*.snap`)) {
    const snapshotContent = await readFile(snapshotPath, "utf-8");
    const info = parseSnapshot(snapshotContent);
    if (!info) continue;

    const input_file = `${project_root}/${info.input_file}`;
    const contentPath = `${snapshotPath}.${info.extension}`;
    const [input, expected] = await Promise.all([
        readFile(input_file, "utf-8"),
        readFile(contentPath, "utf-8"),
    ]);

    test(info.input_file, () => {
        const actual = format(input, info.input_file);
        assert.equal(actual, expected);
    });
}
