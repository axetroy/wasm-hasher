use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    #[cfg(feature = "tracing")]
    tracing_wasm::set_as_global_default();

    Ok(())
}

pub use wasm_hasher_blake::*;
pub use wasm_hasher_md5::*;
pub use wasm_hasher_ripemd::*;
pub use wasm_hasher_sha1::*;
pub use wasm_hasher_sha2::*;
pub use wasm_hasher_sha3::*;
pub use wasm_hasher_sm3::*;
pub use wasm_hasher_tiger::*;
pub use wasm_hasher_whirlpool::*;
