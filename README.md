[![Test](https://github.com/wasm-fmt/ruff_fmt/actions/workflows/test.yml/badge.svg)](https://github.com/wasm-fmt/ruff_fmt/actions/workflows/test.yml)
[![npm](https://img.shields.io/npm/v/@wasm-fmt/ruff_fmt)](https://www.npmjs.com/package/@wasm-fmt/ruff_fmt)

# Install

```bash
npm install @wasm-fmt/ruff_fmt
```

# Usage

```javascript
import init, { format } from "@wasm-fmt/ruff_fmt";

await init();

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

const formatted = format(input);
console.log(formatted);
```

with custom options:

```javascript
import init, { format } from "@wasm-fmt/ruff_fmt";

// ...
const formatted = format(input, {
    indent_style: "space",
    indent_width: 4,
    line_width: 88,
    quote_style: "double",
    magic_trailing_comma: "respect",
});
console.log(formatted);
```

For Vite users:

```JavaScript
import init, { format } from "@wasm-fmt/ruff_fmt/vite";

// ...
```

# dprint plugin

```bash
dprint config add wasm-fmt/ruff_fmt
```

# How does it work?

[Ruff] is an extremely fast Python linter, written in Rust.

This package is a WebAssembly build of Ruff formatter, with a JavaScript wrapper.

[Ruff]: https://github.com/astral-sh/ruff
