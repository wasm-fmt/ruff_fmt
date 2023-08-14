cd $(dirname $0)/..
crates_dir=$(pwd)

cd ../..
wasm-pack build --target=web --scope=wasm-fmt crates/ruff_fmt
cp README.md crates/ruff_fmt/pkg/

cd $crates_dir
# backup ruff_fmt.js
cp ./pkg/ruff_fmt.js ./pkg/ruff_fmt.js.bak

git apply ./patch/ruff_fmt.patch

cp -R ./extra/. ./pkg/

./scripts/package.mjs ./pkg/package.json
