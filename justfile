set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set shell := ["bash", "-cu"]

_default:
	@just --list -u

alias test := test-rust

[group('test')]
test-rust:
	cargo test

[group('test')]
test-bun:
	bun test test_bun

[group('test')]
test-deno:
	deno test test_deno --allow-read --parallel

[group('test')]
test-node:
	node --test "test_node/*.test.mjs"

[group('test')]
test-wasm: test-node test-deno test-bun

[group('test')]
test-all: test-rust test-wasm

[unix]
build:
	wasm-pack build --scope=wasm-fmt .
	cp -R ./extra/. ./pkg/
	node ./scripts/package.mjs ./pkg/package.json

[windows]
build:
	wasm-pack build --scope=wasm-fmt .
	Copy-Item -Recurse -Force ./extra/* ./pkg/
	node ./scripts/package.mjs ./pkg/package.json

fmt:
	cargo fmt --all
	taplo fmt .
	dprint fmt

check:
	cargo check --all
	cargo clippy --all -- -D warnings
	cargo fmt --all --check
	taplo fmt --check .

audit:
	cargo audit

ready:
	just check
	just build
	just test-all
	cd pkg && npm pack --dry-run
	cd pkg && npx jsr publish --dry-run

# Bump version (major, minor, patch) or set specific version
version bump_or_version:
	#!/usr/bin/env bash
	if [[ "{{bump_or_version}}" =~ ^(major|minor|patch)$ ]]; then
		cargo set-version --bump "{{bump_or_version}}"
	else
		cargo set-version "{{bump_or_version}}"
	fi

	VERSION=$(cargo pkgid | sed 's/.*[#@]//')

	git add -A
	git commit -m "${VERSION}"
	git tag -a "v${VERSION}" -m "${VERSION}"
