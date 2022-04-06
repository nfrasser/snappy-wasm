mod utils;

use snap;
use std::io;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compress(input: &str) -> Result<Box<[u8]>, JsError> {
    compress_raw(input.as_bytes())
}

#[wasm_bindgen]
pub fn compress_raw(input: &[u8]) -> Result<Box<[u8]>, JsError> {
    let mut iref = input;
    let mut output: Vec<u8> = Vec::new();
    let mut writer = snap::write::FrameEncoder::new(&mut output);
    match io::copy(&mut iref, &mut writer) {
        Ok(_) => {
            drop(writer);
            Ok(output.into_boxed_slice())
        }
        Err(err) => Err(util::handle_err(err))
    }
}

#[wasm_bindgen]
pub fn decompress(input: &[u8]) -> Result<String, JsError> {
    match util::decompress(input) {
        Ok(decompressed) =>  match String::from_utf8(decompressed) {
            Ok(s) => Ok(s),
            Err(_) => Err(JsError::new("Cannot decompress invalid UTF-8 string")),
        }
        Err(err) => Err(util::handle_err(err))
    }
}

#[wasm_bindgen]
pub fn decompress_raw(input: &[u8]) -> Result<Box<[u8]>, JsError> {
    match util::decompress(input) {
        Ok(output) => Ok(output.into_boxed_slice()),
        Err(err) => Err(util::handle_err(err))
    }
}

mod util {
    use std::io;

    pub fn decompress(input: &[u8]) -> io::Result<Vec<u8>> {
        let mut reader = snap::read::FrameDecoder::new(input);
        let mut output: Vec<u8> = Vec::new();
        io::copy(&mut reader, &mut output)?;
        Ok(output)
    }

    pub fn handle_err(err: io::Error) -> super::JsError {
        super::JsError::new(err.to_string().as_str())
    }
}
