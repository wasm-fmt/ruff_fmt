import type { ConfigHandle as BridgeConfigHandle } from "@wasm-fmt/runtime";
import type { Config } from "./ruff_fmt_config.d.ts";
export type * from "./ruff_fmt_config.d.ts";

export type ConfigHandle = BridgeConfigHandle<"ruff_fmt">;
export type ConfigInput = Config | ConfigHandle;
export interface TextRange {
	/** Inclusive UTF-8 byte offset in the original source. */
	readonly start: number;
	/** Exclusive UTF-8 byte offset in the original source. */
	readonly end: number;
}

/** Format the entire Python source code string. */
export declare function format(input: string, config?: ConfigInput | null): string;
/** Format the entire Python source code string. */
export declare function format(input: string, path?: string | null, config?: ConfigInput | null): string;
/** Format one range, expressed as UTF-8 byte offsets in the original source. */
export declare function formatRanges(input: string, ranges: readonly TextRange[], config?: ConfigInput | null): string;
/** Format one range, expressed as UTF-8 byte offsets in the original source. */
export declare function formatRanges(
	input: string,
	ranges: readonly TextRange[],
	path?: string | null,
	config?: ConfigInput | null,
): string;
/** Create a reusable formatter configuration. */
export declare function createConfig(config?: Config): ConfigHandle;
/** Release a reusable formatter configuration. */
export declare function releaseConfig(handle: ConfigHandle): void;
