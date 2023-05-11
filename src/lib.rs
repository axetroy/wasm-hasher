use js_sys::{Function, Uint8Array};
use wasm_bindgen::prelude::*;
use md5::Md5;
use wasm_bindgen_futures::JsFuture;
use web_sys::Blob;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use sm3::Sm3;
use digest::Digest;

#[wasm_bindgen]
pub async fn md5(blob: Blob, cb: Function) -> Result<String, JsValue> {
    let mut hasher = Md5::new();
    computed(blob, cb, &mut hasher).await;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha2_224(blob: Blob, cb: Function) -> Result<String, JsValue> {
    let mut hasher = Sha224::new();
    computed(blob, cb, &mut hasher).await;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha2_256(blob: Blob, cb: Function) -> Result<String, JsValue> {
    let mut hasher = Sha256::new();
    computed(blob, cb, &mut hasher).await;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha2_384(blob: Blob, cb: Function) -> Result<String, JsValue> {
    let mut hasher = Sha384::new();
    computed(blob, cb, &mut hasher).await;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha2_512(blob: Blob, cb: Function) -> Result<String, JsValue> {
    let mut hasher = Sha512::new();
    computed(blob, cb, &mut hasher).await;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha1(blob: Blob, cb: Function) -> Result<String, JsValue> {
    let mut hasher = Sha1::new();
    computed(blob, cb, &mut hasher).await;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha3_224(blob: Blob, cb: Function) -> Result<String, JsValue> {
    let mut hasher = Sha3_224::new();
    computed(blob, cb, &mut hasher).await;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha3_256(blob: Blob, cb: Function) -> Result<String, JsValue> {
    let mut hasher = Sha3_256::new();
    computed(blob, cb, &mut hasher).await;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha3_384(blob: Blob, cb: Function) -> Result<String, JsValue> {
    let mut hasher = Sha3_384::new();
    computed(blob, cb, &mut hasher).await;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sha3_512(blob: Blob, cb: Function) -> Result<String, JsValue> {
    let mut hasher = Sha3_512::new();
    computed(blob, cb, &mut hasher).await;
    Ok(format!("{:x}", hasher.finalize()))
}

#[wasm_bindgen]
pub async fn sm3(blob: Blob, cb: Function) -> Result<String, JsValue> {
    let mut hasher = Sm3::new();
    computed(blob, cb, &mut hasher).await;
    Ok(format!("{:x}", hasher.finalize()))
}

pub async fn computed<D: Digest>(blob: Blob, cb: Function, hasher: &mut D) {
    let step: f64 = 10485760.0;
    let size = blob.size();
    let chunks = (size / step).ceil() as i64;
    let mut start: f64 = 0.0;
    for _ in 0..chunks {
        let mut end = start + step;
        end = if end >= size {
            size
        } else {
            end
        };
        let data = blob.slice_with_f64_and_f64(start, end).expect("slice blob failed!");
        let buffer: JsValue = JsFuture::from(data.array_buffer()).await.expect("get arrayBuffer failed!");
        hasher.update(&Uint8Array::new(&buffer).to_vec());
        let _ = cb.call1(&JsValue::null(), &JsValue::from(start / size * 100.0));
        start += step;
    };
}


