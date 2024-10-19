import { assertEquals } from "jsr:@std/assert";
import { walk } from "jsr:@std/fs/walk";

import init, { format } from "../pkg/ruff_fmt.js";

await init();

const test_root = new URL(import.meta.resolve("../test_data"));
Deno.chdir(test_root);

for await (const entry of walk(".", {
    includeDirs: false,
    exts: ["py", "pyi"],
})) {
    if (entry.name.startsWith(".")) {
        continue;
    }

    const input_path = entry.path;
    const expect_path = input_path + ".expect";

    const input = Deno.readTextFileSync(input_path);
    const expected = Deno.readTextFileSync(expect_path);

    Deno.test(input_path, () => {
        const actual = format(input, entry.path);
        assertEquals(actual, expected);
    });
}
