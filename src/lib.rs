// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use wasm_hasher_md5::*;
pub use wasm_hasher_sha1::*;
pub use wasm_hasher_sha2::*;
pub use wasm_hasher_sha3::*;
pub use wasm_hasher_sm3::*;
pub use wasm_hasher_ripemd::*;
pub use wasm_hasher_tiger::*;
