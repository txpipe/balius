pub mod wit {
    wit_bindgen::generate!({
        pub_export_macro: true,
        default_bindings_module: "balius_sdk::wit",
        path: "./wit",
        additional_derives: [PartialEq, Eq, Hash, Clone, serde::Serialize,serde::Deserialize]
    });
}

/// Re-export of the macros crate
pub use balius_macros as macros;

/// Macro to mark the main function for the worker
pub use balius_macros::main;

/// Transaction builder artifacts
pub mod txbuilder;

/// Internal functions to be used by the generated code
pub mod _internal;

/// Quality of life features to make the SDK more ergonomic
mod qol;

/// A tracing implementation which sends logs to the server
pub mod logging;

pub use _internal::Worker;
pub use qol::*;
