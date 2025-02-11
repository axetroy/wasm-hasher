use digest::Digest;
use js_sys::{Error, Function, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AbortSignal, Blob};

pub async fn computed<D: Digest>(
    signal: AbortSignal,
    blob: Blob,
    chunk_size: Option<f64>,
    cb: &Option<Function>,
    hasher: &mut D,
) -> Result<(), JsValue> {
    let size = blob.size();
    let chunk = chunk_size.unwrap_or((1024 * 1024 * 5) as f64); // 默认 5MB
    let chunks = (size / chunk).ceil() as i64;
    let mut start: f64 = 0.0;

    // 预分配一个 Uint8Array 用于存储数据
    let mut buffer = Vec::with_capacity(chunk as usize);

    for _ in 0..chunks {
        if signal.aborted() {
            return Err(Error::new("Signal has been aborted!").into());
        }

        let end = (start + chunk).min(size);
        let data = blob
            .slice_with_f64_and_f64(start, end)
            .map_err(|e| Error::new(&format!("slice blob failed: {:?}", e)))?;

        // 复用 buffer 以减少内存分配
        buffer.clear();
        let array_buffer = JsFuture::from(data.array_buffer())
            .await
            .map_err(|e| Error::new(&format!("get arrayBuffer failed: {:?}", e)))?;
        let uint8_array = Uint8Array::new(&array_buffer);
        uint8_array.copy_to(&mut buffer);

        hasher.update(&buffer);

        // 调用回调函数，更新进度
        if let Some(func) = cb {
            let progress = (start / size * 100.0).min(100.0); // 确保进度不超过 100%
            let _ = func.call1(&JsValue::null(), &JsValue::from(progress));
        }

        start += chunk;
    }

    // 最终回调，确保进度为 100%
    if let Some(func) = cb {
        let _ = func.call1(&JsValue::null(), &JsValue::from(100.0));
    }

    Ok(())
}
