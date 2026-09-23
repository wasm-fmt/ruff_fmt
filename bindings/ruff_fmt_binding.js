// @ts-check

const encoder = new TextEncoder();

/**
 * @type {import("@wasm-fmt/runtime").FormatterAdapter<
 *   typeof import("./ruff_fmt.d.ts")
 * >}
 */
const adapter = {
	create(wasm, host) {
		const runtime = host.createRuntime(wasm, { encodeConfig });

		/** @type {typeof import("./ruff_fmt.d.ts")} */
		const api = {
			/**
			 * @param {string} source
			 * @param {string | import("./ruff_fmt.d.ts").ConfigInput | null} [filenameOrConfig]
			 * @param {import("./ruff_fmt.d.ts").ConfigInput | null} [config]
			 */
			format(source, filenameOrConfig, config) {
				return runtime.format(source, filenameOrConfig ?? undefined, config ?? undefined);
			},
			/**
			 * @param {string} source
			 * @param {readonly import("./ruff_fmt.d.ts").TextRange[]} ranges
			 * @param {string | import("./ruff_fmt.d.ts").ConfigInput | null} [filenameOrConfig]
			 * @param {import("./ruff_fmt.d.ts").ConfigInput | null} [config]
			 */
			formatRanges(source, ranges, filenameOrConfig, config) {
				return runtime.formatRanges(source, ranges, filenameOrConfig ?? undefined, config ?? undefined);
			},
			/** @param {import("./ruff_fmt_config.d.ts").Config} [config] */
			createConfig(config) {
				return /** @type {import("./ruff_fmt.d.ts").ConfigHandle} */ (runtime.createConfig(config ?? {}));
			},
			/** @param {import("./ruff_fmt.d.ts").ConfigHandle} handle */
			releaseConfig(handle) {
				return runtime.releaseConfig(handle);
			},
		};

		return api;
	},
};

export default adapter;

/**
 * @param {unknown} config
 * @return {Uint8Array}
 */
function encodeConfig(config) {
	if (typeof config === "string") {
		return encoder.encode(config);
	}

	const json = JSON.stringify(config);
	if (json === undefined) {
		throw new TypeError("config must be JSON serializable");
	}
	return encoder.encode(json);
}
