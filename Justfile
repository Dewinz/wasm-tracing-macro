# Don't use `https://`, just the URL without protocol.
build-and-push registry_url version: build-wasm-components
	cd component && wash push {{registry_url}}/component:{{version}} build/otel_wasm_component.wasm
	cd ..
	cd http-api && wash push {{registry_url}}/http-api:{{version}} build/http_api.wasm
	cd ..

build-wasm-components:
	#!/usr/bin/env bash
	cd component && wash build
	cd ..
	cd http-api && wash build
	cd ..
