wasm-pack build --target=web --scope=wasm-fmt crates/ruff_fmt

cd crates/ruff_fmt

# backup ruff_fmt.js
cp ./pkg/ruff_fmt.js ./pkg/ruff_fmt.js.bak

git apply ./patch/ruff_fmt.patch

cp -r ./extra/ ./pkg/

./scripts/package.mjs ./pkg/package.json
