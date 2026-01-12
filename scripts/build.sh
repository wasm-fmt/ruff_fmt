cd $(dirname $0)/..
project_dir=$(pwd)

wasm-pack build --target=web --scope=wasm-fmt
cp README.md pkg/

cd $project_dir

cp -R ./extra/. ./pkg/

./scripts/package.mjs ./pkg/package.json
