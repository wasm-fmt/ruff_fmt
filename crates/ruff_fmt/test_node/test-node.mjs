import init, { format } from "../pkg/ruff_fmt.js";
import { test } from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";

await init();

const files = fs
    .readdirSync(new URL("../test_data", import.meta.url), "utf-8")
    .filter((f) => f.endsWith(".input"))
    .map((f) => {
        return {
            input_name: f,
            expect_name: f.replace(".input", ".expect"),
        };
    });

for (const { input_name, expect_name } of files) {
    test(input_name, () => {
        const input = fs.readFileSync(
            new URL(`../test_data/${input_name}`, import.meta.url),
            "utf-8"
        );
        const expected = fs.readFileSync(
            new URL(`../test_data/${expect_name}`, import.meta.url),
            "utf-8"
        );
        const actual = format(input);

        assert.equal(actual, expected);
    });
}
