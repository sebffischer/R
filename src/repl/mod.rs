mod release;

#[cfg(feature = "repl")]
mod core;
pub use core::*;

#[cfg(feature = "repl")]
pub mod highlight;

#[cfg(feature = "repl")]
pub mod prompt;

#[cfg(feature = "repl")]
pub mod validator;

#[cfg(target_family = "wasm")]
pub mod headless;
