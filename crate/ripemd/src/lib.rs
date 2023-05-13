use js_sys::Function;
use ripemd::{Ripemd128, Ripemd160, Ripemd256, Ripemd320, Digest};
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
pub async fn ripemd_128(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Ripemd128::new();
    match wasm_hasher_lib::computed(signal, blob, chunk, &on_progress, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

#[wasm_bindgen]
pub async fn ripemd_160(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Ripemd160::new();
    match wasm_hasher_lib::computed(signal, blob, chunk, &on_progress, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

#[wasm_bindgen]
pub async fn ripemd_256(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Ripemd256::new();
    match wasm_hasher_lib::computed(signal, blob, chunk, &on_progress, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

#[wasm_bindgen]
pub async fn ripemd_320(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Ripemd320::new();
    match wasm_hasher_lib::computed(signal, blob, chunk, &on_progress, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}