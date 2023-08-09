import init, { format } from "../pkg/ruff_fmt.js";

import { assertEquals } from "https://deno.land/std@0.197.0/assert/mod.ts";

await init();

// Deno glob files
const files = [...Deno.readDirSync(new URL("../test_data", import.meta.url))]
    .filter((f) => f.isFile && f.name.endsWith(".input"))
    .map((f) => {
        return {
            input_name: f.name,
            expect_name: f.name.replace(".input", ".expect"),
        };
    });

for (const { input_name, expect_name } of files) {
    Deno.test(input_name, () => {
        const input = Deno.readTextFileSync(
            new URL(`../test_data/${input_name}`, import.meta.url)
        );
        const expected = Deno.readTextFileSync(
            new URL(`../test_data/${expect_name}`, import.meta.url)
        );

        const actual = format(input);

        assertEquals(actual, expected);
    });
}
