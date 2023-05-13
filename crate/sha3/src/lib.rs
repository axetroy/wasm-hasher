use digest::Digest;
use js_sys::Function;
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use wasm_bindgen::prelude::*;
use web_sys::{AbortSignal, Blob};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub async fn sha3_224(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Sha3_224::new();
    wasm_hasher_lib::computed(signal, blob, chunk, &on_progress, &mut hasher).await?;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha3_256(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Sha3_256::new();
    wasm_hasher_lib::computed(signal, blob, chunk, &on_progress, &mut hasher).await?;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha3_384(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Sha3_384::new();
    wasm_hasher_lib::computed(signal, blob, chunk, &on_progress, &mut hasher).await?;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha3_512(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Sha3_512::new();
    wasm_hasher_lib::computed(signal, blob, chunk, &on_progress, &mut hasher).await?;
    Ok(format!("{:x}", hasher.finalize()))
}
