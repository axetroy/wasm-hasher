use digest::Digest;
use js_sys::Function;
use sha1::Sha1;
use wasm_bindgen::prelude::*;
use web_sys::{AbortSignal, Blob};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "start")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    #[cfg(feature = "tracing")]
    tracing_wasm::set_as_global_default();

    Ok(())
}

#[wasm_bindgen]
pub async fn sha1(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Sha1::new();
    wasm_hasher_lib::computed(signal, blob, chunk, &on_progress, &mut hasher).await?;
    Ok(format!("{:x}", hasher.finalize()))
}
