pub mod runtime;
pub mod host_functions;
pub mod demo;
pub use runtime::PluginRuntime;
// Keep your existing exports, just add:
pub mod security;