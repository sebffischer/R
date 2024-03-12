use crate::parser::Localization;

#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};

#[cfg_attr(
    feature = "wasm",
    wasm_bindgen::prelude::wasm_bindgen,
    derive(Serialize, Deserialize),
    serde(rename_all(serialize = "kebab-case", deserialize = "kebab-case"))
)]
#[derive(Debug, Copy, Clone, PartialEq, clap::ValueEnum, strum::EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Experiment {
    TailCalls,
    RestArgs,
}

/// Run the R REPL
#[cfg_attr(
    feature = "wasm",
    wasm_bindgen::prelude::wasm_bindgen(getter_with_clone),
    derive(Serialize, Deserialize),
    serde(rename_all(serialize = "kebab-case", deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "wasm"), derive(clap::Parser))]
#[derive(Debug, Clone, Default)]
pub struct Cli {
    /// Localization to use for runtime
    #[cfg_attr(not(feature = "wasm"), arg(short, long, default_value = "en"))]
    pub locale: Localization,

    /// Show the extended warranty information at startup
    #[cfg_attr(not(feature = "wasm"), arg(long))]
    pub warranty: bool,

    /// Enable experimental language features
    #[cfg_attr(not(feature = "wasm"), arg(short = 'x', long, value_delimiter = ','))]
    pub experiments: Vec<Experiment>,
}
