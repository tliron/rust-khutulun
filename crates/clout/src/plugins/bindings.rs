#![allow(missing_docs)]

wasmtime::component::bindgen!({
    path: "../../assets/wit/clout-plugins.wit",
    with: {
        "clout:plugins/host/value-list": super::host::List,
        "clout:plugins/host/value-map": super::host::Map,
    },
    trappable_imports: true,
});
