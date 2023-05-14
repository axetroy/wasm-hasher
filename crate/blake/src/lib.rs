use blake_hash::{Blake224, Blake256, Blake384, Blake512, Digest};
use js_sys::{Function, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
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
pub async fn blake_224(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Blake224::new();
    computed(signal, blob, chunk, &on_progress, &mut hasher).await?;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn blake_256(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Blake256::new();
    computed(signal, blob, chunk, &on_progress, &mut hasher).await?;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn blake_384(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Blake384::new();
    computed(signal, blob, chunk, &on_progress, &mut hasher).await?;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn blake_512(
    signal: AbortSignal,
    blob: Blob,
    chunk: Option<f64>,
    on_progress: Option<Function>,
) -> Result<String, JsValue> {
    let mut hasher = Blake512::new();
    computed(signal, blob, chunk, &on_progress, &mut hasher).await?;
    Ok(format!("{:x}", hasher.finalize()))
}

pub async fn computed<D: Digest>(
    signal: AbortSignal,
    blob: Blob,
    chunk_size: Option<f64>,
    cb: &Option<Function>,
    hasher: &mut D,
) -> Result<(), JsValue> {
    let size = blob.size();

    let chunk = match chunk_size {
        Some(size) => size,
        None => (1024 * 1024 * 5) as f64,
    };

    let chunks = (size / chunk).ceil() as i64;
    let mut start: f64 = 0.0;

    for _ in 0..chunks {
        if signal.aborted() {
            return Err(JsValue::from("Signal has been abort!"));
        }

        let mut end = start + chunk;
        end = if end >= size { size } else { end };
        let data = blob
            .slice_with_f64_and_f64(start, end)
            .expect("slice blob failed!");
        let buffer: JsValue = JsFuture::from(data.array_buffer())
            .await
            .expect("get arrayBuffer failed!");
        hasher.update(&Uint8Array::new(&buffer).to_vec());
        match cb {
            Some(func) => {
                let _ = func.call1(&JsValue::null(), &JsValue::from(start / size * 100.0));
            }
            None => (),
        }
        start += chunk;
    }

    match cb {
        Some(func) => {
            let _ = func.call1(&JsValue::null(), &JsValue::from(100.0));
        }
        None => (),
    }

    Ok(())
}
