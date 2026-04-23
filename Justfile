apply registry_url version: (update registry_url version)
	kubectl delete workloaddeployments -n wasmcloud-system --all
	kubectl apply -f workload.yaml -n wasmcloud-system

update registry_url version: (build-and-push registry_url version) (update-workload version)

# Don't use `https://`, just the URL without protocol.
build-and-push registry_url version: build-wasm-components
	wash oci push --insecure {{registry_url}}/component:{{version}} component/target/wasm32-wasip2/release/otel_wasm_component.wasm
	wash oci push --insecure {{registry_url}}/data:{{version}} data/target/wasm32-wasip2/release/data.wasm
	wash oci push --insecure {{registry_url}}/http-api:{{version}} http-api/target/wasm32-wasip2/release/http_api.wasm

update-workload version:
	sed -i 's|image: \(.*http-api:\).*|image: \1{{version}}|' workload.yaml
	sed -i 's|image: \(.*data:\).*|image: \1{{version}}|' workload.yaml
	sed -i 's|image: \(.*component:\).*|image: \1{{version}}|' workload.yaml

build-wasm-components:
	-cd component && wash build
	cd data && cargo build --release --target=wasm32-wasip2
	-cd http-api && wash build

format:
	cd component && cargo fmt
	cd data && cargo fmt
	cd http-api && cargo fmt

quality:
	cd component && cargo clippy
	cd data && cargo clippy
	cd http-api && cargo clippy
