use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator for smaller bundle size
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
pub struct FormatResult {
    pub success: bool,
    pub formatted: Option<String>,
    pub error: Option<String>,
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn format_typescript(code: &str) -> String {
    init_panic_hook();

    // Use krokfmt to format the TypeScript code
    let result = match krokfmt::format_typescript(code, "playground.ts") {
        Ok(formatted) => FormatResult {
            success: true,
            formatted: Some(formatted),
            error: None,
        },
        Err(err) => FormatResult {
            success: false,
            formatted: None,
            error: Some(format!("{err}")),
        },
    };

    serde_json::to_string(&result).unwrap_or_else(|e| {
        let error_result = FormatResult {
            success: false,
            formatted: None,
            error: Some(format!("Serialization error: {e}")),
        };
        serde_json::to_string(&error_result).unwrap_or_default()
    })
}

#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
