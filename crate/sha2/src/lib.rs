use digest::Digest;
use js_sys::Function;
use sha2::{Sha224, Sha256, Sha384, Sha512};
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
pub async fn sha2_224(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Sha224::new();
    wasm_hasher_lib::computed(signal, blob, chunk, &on_progress, &mut hasher).await?;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha2_256(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Sha256::new();
    wasm_hasher_lib::computed(signal, blob, chunk, &on_progress, &mut hasher).await?;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha2_384(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Sha384::new();
    wasm_hasher_lib::computed(signal, blob, chunk, &on_progress, &mut hasher).await?;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha2_512(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Sha512::new();
    wasm_hasher_lib::computed(signal, blob, chunk, &on_progress, &mut hasher).await?;
    Ok(format!("{:x}", hasher.finalize()))
}
