wasm-pack build -t web --scope=wasm-fmt
cp ./pkg/ruff_fmt.js ./pkg/ruff_fmt.js.bak

git apply ./scripts/ruff_fmt.patch

cp ./scripts/vite.js ./scripts/.npmignore ./pkg/

./scripts/package.mjs ./pkg/package.json
