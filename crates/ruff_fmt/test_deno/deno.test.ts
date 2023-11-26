/// <reference lib="deno.ns" />
import { assertEquals } from "https://deno.land/std@0.201.0/assert/mod.ts";
import { walk } from "https://deno.land/std@0.201.0/fs/walk.ts";
import { relative } from "https://deno.land/std@0.201.0/path/mod.ts";
import init, { format } from "../pkg/ruff_fmt.js";

await init();

const update = Deno.args.includes("--update");

const test_root = new URL("../test_data", import.meta.url);

for await (const entry of walk(test_root, {
    includeDirs: false,
    exts: ["input"],
})) {
    const expect_path = entry.path.replace(/input$/, "expect");
    const input = Deno.readTextFileSync(entry.path);

    if (update) {
        const actual = format(input);
        Deno.writeTextFileSync(expect_path, actual);
    } else {
        const expected = Deno.readTextFileSync(expect_path);

        const test_name = relative(test_root.pathname, entry.path);

        Deno.test(test_name, () => {
            const actual = format(input);
            assertEquals(actual, expected);
        });
    }
}
