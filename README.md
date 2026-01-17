[![Test](https://github.com/wasm-fmt/ruff_fmt/actions/workflows/test.yml/badge.svg)](https://github.com/wasm-fmt/ruff_fmt/actions/workflows/test.yml)

# Install

[![npm](https://img.shields.io/npm/v/@wasm-fmt/ruff_fmt?color=3572A5)](https://www.npmjs.com/package/@wasm-fmt/ruff_fmt)

```bash
npm install @wasm-fmt/ruff_fmt
```

[![jsr.io](https://jsr.io/badges/@fmt/ruff-fmt?color=3572A5)](https://jsr.io/@fmt/ruff-fmt)

```bash
npx jsr add @fmt/ruff-fmt
```

# Usage

## Node.js / Deno / Bun / Bundler

```javascript
import { format } from "@wasm-fmt/ruff_fmt";

const input = `x = {  'a':37,'b':42,

'c':927}

y = 'hello ''world'
z = 'hello '+'world'
a = 'hello {}'.format('world')
class foo  (     object  ):
  def f    (self   ):
    return       37*-+2
  def g(self, x,y=42):
      return y
def f  (   a ) :
  return      37+-+a[42-x :  y**3]`;

const formatted = format(input, "main.py", {
	indent_style: "space",
	indent_width: 4,
	line_width: 88,
	quote_style: "double",
	magic_trailing_comma: "respect",
});
console.log(formatted);
```

## Node.js < 22.19

```JavaScript
import { format } from "@wasm-fmt/ruff_fmt/node";
```

## Web

For web environments, you need to initialize WASM module manually:

```javascript
import init, { format } from "@wasm-fmt/ruff_fmt/web";

await init();

const input = `x = {  'a':37,'b':42,

'c':927}`;

const formatted = format(input, "main.py");
console.log(formatted);
```

### Vite

```JavaScript
import init, { format } from "@wasm-fmt/ruff_fmt/vite";

await init();
// ...
```

Add `"@wasm-fmt/ruff_fmt"` to `optimizeDeps.exclude` in your vite config:

```JSON
{
	"optimizeDeps": {
		"exclude": ["@wasm-fmt/ruff_fmt"]
	}
}
```

## Entry Points

- `.` - Auto-detects environment (Node.js uses node, Webpack uses bundler, default is ESM)
- `./node` - Node.js environment (no init required)
- `./esm` - ESM environments like Deno (no init required)
- `./bundler` - Bundlers like Webpack (no init required)
- `./web` - Web browsers (requires manual init)
- `./vite` - Vite bundler (requires manual init)

# dprint plugin

> [!NOTE]
> dpint plugin is deprecated, please use https://dprint.dev/plugins/ruff instead.

# How does it work?

[Ruff] is an extremely fast Python linter, written in Rust.

This package is a WebAssembly build of Ruff formatter, with a JavaScript wrapper.

[Ruff]: https://github.com/astral-sh/ruff
