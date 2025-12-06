import { Glob } from "bun";
import { expect, test } from "bun:test";
import { fileURLToPath } from "node:url";
import { parseSnapshot } from "../test_utils";

import init, { format } from "../pkg/ruff_fmt";

await init();

const project_root = fileURLToPath(import.meta.resolve("../../.."));
const snapshots_root = fileURLToPath(import.meta.resolve("../snapshots"));

const glob = new Glob("*.snap");

for await (const snap_path of glob.scan({ cwd: snapshots_root })) {
    // Only process metadata files (not the .snap.py content files)
    const snapshotPath = `${snapshots_root}/${snap_path}`;
    const snapshotContent = await Bun.file(snapshotPath).text();
    const info = parseSnapshot(snapshotContent);
    if (!info) continue;

    const input_file = Bun.file(`${project_root}/${info.input_file}`);
    const expected_file = Bun.file(`${snapshotPath}.${info.extension}`);

    const [input, expected] = await Promise.all([
        input_file.text(),
        expected_file.text(),
    ]);

    test(info.input_file, () => {
        const actual = format(input, info.input_file);
        expect(actual).toBe(expected);
    });
}
