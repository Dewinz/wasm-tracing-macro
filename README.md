# WASM Tracing Macro
This macro automatically adds OTel tracing exposed by wasmCloud to your functions.

To use this you need to also tell wasmCloud to expose the OTel tracing interface in the `workload.yaml`, like so:

apiVersion: runtime.wasmcloud.dev/v1alpha1
```YAML
kind: WorkloadDeployment
metadata:
  name: test
spec:
  replicas: 1
  template:
    spec:
      hostSelector:
        hostgroup: default
      components:
        - name: http-api
          image: some-registry:5000/something:v1
      # This is the important part.
      hostInterfaces:
        - namespace: wasi
          package: otel
          interfaces:
            - tracing
```

You can either just apply the macro like so if you have used `wit_bindgen::generate!()` in the current module like so:
```Rust
#[trace]
fn handle_home() -> (u16, String) {
    component::first();
    component::second(&String::default());
    another_function();
    (200, String::from("Hi"))
}
```
Or, if the `wit_bindgen::generate!()` has been ran in a separate module you can pass the module like so, the module that `wit_bindgen::generate!()` has been run in here is `bindings`:
```Rust
#[trace(bindings)]
fn handle_home() -> (u16, String) {
    component::first();
    component::second(&String::default());
    another_function();
    (200, String::from("Hi"))
}
```

