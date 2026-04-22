update registry_url version: (build-and-push registry_url version) (update-workload version)

# Don't use `https://`, just the URL without protocol.
build-and-push registry_url version: build-wasm-components
	wash push --insecure {{registry_url}}/component:{{version}} component/build/otel_wasm_component.wasm
	wash push --insecure {{registry_url}}/http-api:{{version}} http-api/build/http_api.wasm

update-workload version:
	sed -i 's|image: \(.*http-api:\).*|image: \1{{version}}|' workload.yaml
	sed -i 's|image: \(.*component:\).*|image: \1{{version}}|' workload.yaml

build-wasm-components:
	-cd component && wash build
	-cd http-api && wash build
