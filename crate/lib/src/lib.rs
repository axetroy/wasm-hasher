use digest::Digest;
use js_sys::{Function, Uint8Array};
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
