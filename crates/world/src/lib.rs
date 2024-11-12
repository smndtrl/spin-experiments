#![allow(missing_docs)]
#![allow(non_camel_case_types)] // bindgen emits Host_Pre and Host_Indices

pub use async_trait::async_trait;

wasmtime::component::bindgen!({
    inline: r#"
    package smndtrl:runtime;
    world host {
        include smndtrl:experiments/counter;
    }
    "#,
    path: "../../wit",
    async: true,
    // The following is a roundabout way of saying "the host implementations for these interfaces don't trap"
    trappable_error_type: {
        "smndtrl:experiments/data/error" => smndtrl::experiments::data::Error,
    },
    trappable_imports: true,
});
