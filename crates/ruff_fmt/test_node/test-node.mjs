import assert from "node:assert/strict";
import fs from "node:fs/promises";
import { basename } from "node:path";
import { chdir } from "node:process";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

import init, { format } from "../pkg/ruff_fmt_node.js";

await init();

const test_root = fileURLToPath(import.meta.resolve("../test_data"));
chdir(test_root);

for await (const input_path of fs.glob("**/*.{py,pyi}")) {
    if (basename(input_path).startsWith(".")) {
        continue;
    }

    const expect_path = input_path + ".expect";

    const [input, expected] = await Promise.all([
        fs.readFile(input_path, "utf-8"),
        fs.readFile(expect_path, "utf-8"),
    ]);

    test(input_path, () => {
        const actual = format(input, input_path);
        assert.equal(actual, expected);
    });
}
