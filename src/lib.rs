mod utils;

use wasm_bindgen::prelude::*;
use std::io::copy;
use snap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compress(input: &[u8]) -> Option<Box<[u8]>> {
    let mut iref = input;
    let mut output: Vec<u8> = Vec::new();
    let mut writer = snap::write::FrameEncoder::new(&mut output);
    if let Ok(_) = copy(&mut iref, &mut writer) {
        drop(writer);
        Some(output.into_boxed_slice())
    } else {
        None
    }
}


#[wasm_bindgen]
pub fn compress_str(input: &str) -> Option<Box<[u8]>> {
    compress(input.as_bytes())
}


#[wasm_bindgen]
pub fn decompress(input: &[u8]) -> Option<Box<[u8]>> {
    if let Some(output) = util::decompress(input) {
        Some(output.into_boxed_slice())
    } else {
        None
    }
}

#[wasm_bindgen]
pub fn decompress_str(input: &[u8]) -> Result<String, JsValue> {
    if let Some(decompressed) = util::decompress(input) {
        match String::from_utf8(decompressed) {
            Ok(s) => Ok(s),
            Err(_) => Err(JsValue::from("Invalid string"))
        }
    } else {
        Err(JsValue::from("Could not decompress given data"))
    }
}

mod util {
    pub fn decompress(input: &[u8]) -> Option<Vec<u8>> {
        let mut reader = snap::read::FrameDecoder::new(input);
        let mut output: Vec<u8> = Vec::new();
        if let Ok(_) = super::copy(&mut reader, &mut output) {
            Some(output)
        } else {
            None
        }
    }
}
