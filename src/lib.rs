use digest::Digest;
use js_sys::{Function, Uint8Array};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use sm3::Sm3;
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

#[wasm_bindgen]
pub async fn md5(
    signal: AbortSignal,
    blob: Blob,
    chunk: f64,
    cb: Function,
) -> Result<String, JsValue> {
    let mut hasher = Md5::new();
    match computed(signal, blob, chunk, cb, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

#[wasm_bindgen]
pub async fn sha2_224(
    signal: AbortSignal,
    blob: Blob,
    chunk: f64,
    cb: Function,
) -> Result<String, JsValue> {
    let mut hasher = Sha224::new();
    match computed(signal, blob, chunk, cb, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

#[wasm_bindgen]
pub async fn sha2_256(
    signal: AbortSignal,
    blob: Blob,
    chunk: f64,
    cb: Function,
) -> Result<String, JsValue> {
    let mut hasher = Sha256::new();
    match computed(signal, blob, chunk, cb, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

#[wasm_bindgen]
pub async fn sha2_384(
    signal: AbortSignal,
    blob: Blob,
    chunk: f64,
    cb: Function,
) -> Result<String, JsValue> {
    let mut hasher = Sha384::new();
    match computed(signal, blob, chunk, cb, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

#[wasm_bindgen]
pub async fn sha2_512(
    signal: AbortSignal,
    blob: Blob,
    chunk: f64,
    cb: Function,
) -> Result<String, JsValue> {
    let mut hasher = Sha512::new();
    match computed(signal, blob, chunk, cb, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

#[wasm_bindgen]
pub async fn sha1(
    signal: AbortSignal,
    blob: Blob,
    chunk: f64,
    cb: Function,
) -> Result<String, JsValue> {
    let mut hasher = Sha1::new();
    match computed(signal, blob, chunk, cb, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

#[wasm_bindgen]
pub async fn sha3_224(
    signal: AbortSignal,
    blob: Blob,
    chunk: f64,
    cb: Function,
) -> Result<String, JsValue> {
    let mut hasher = Sha3_224::new();
    match computed(signal, blob, chunk, cb, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

#[wasm_bindgen]
pub async fn sha3_256(
    signal: AbortSignal,
    blob: Blob,
    chunk: f64,
    cb: Function,
) -> Result<String, JsValue> {
    let mut hasher = Sha3_256::new();
    match computed(signal, blob, chunk, cb, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

#[wasm_bindgen]
pub async fn sha3_384(
    signal: AbortSignal,
    blob: Blob,
    chunk: f64,
    cb: Function,
) -> Result<String, JsValue> {
    let mut hasher = Sha3_384::new();
    match computed(signal, blob, chunk, cb, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

#[wasm_bindgen]
pub async fn sha3_512(
    signal: AbortSignal,
    blob: Blob,
    chunk: f64,
    cb: Function,
) -> Result<String, JsValue> {
    let mut hasher = Sha3_512::new();
    match computed(signal, blob, chunk, cb, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

#[wasm_bindgen]
pub async fn sm3(
    signal: AbortSignal,
    blob: Blob,
    chunk: f64,
    cb: Function,
) -> Result<String, JsValue> {
    let mut hasher = Sm3::new();
    match computed(signal, blob, chunk, cb, &mut hasher).await {
        Ok(_) => Ok(format!("{:x}", hasher.finalize())),
        Err(err) => Err(err),
    }
}

pub async fn computed<D: Digest>(
    signal: AbortSignal,
    blob: Blob,
    chunk: f64,
    cb: Function,
    hasher: &mut D,
) -> Result<(), JsValue> {
    let size = blob.size();
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
        let _ = cb.call1(&JsValue::null(), &JsValue::from(start / size * 100.0));
        start += chunk;
    }

    let _ = cb.call1(&JsValue::null(), &JsValue::from(100.0));

    Ok(())
}
