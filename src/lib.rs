mod utils;

use snap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compress(input: &str) -> Result<Box<[u8]>, JsError> {
    compress_raw(input.as_bytes())
}

#[wasm_bindgen]
pub fn compress_raw(input: &[u8]) -> Result<Box<[u8]>, JsError> {
    let mut encoder = snap::raw::Encoder::new();
    match encoder.compress_vec(input) {
        Ok(output) => Ok(output.into_boxed_slice()),
        Err(err) => Err(util::handle_err(err)),
    }
}

#[wasm_bindgen]
pub fn decompress(input: &[u8]) -> Result<String, JsError> {
    match util::decompress(input) {
        Ok(decompressed) => match String::from_utf8(decompressed) {
            Ok(s) => Ok(s),
            Err(_) => Err(JsError::new("Cannot decompress invalid UTF-8 string")),
        },
        Err(err) => Err(util::handle_err(err)),
    }
}

#[wasm_bindgen]
pub fn decompress_raw(input: &[u8]) -> Result<Box<[u8]>, JsError> {
    match util::decompress(input) {
        Ok(output) => Ok(output.into_boxed_slice()),
        Err(err) => Err(util::handle_err(err)),
    }
}

mod util {
    pub fn decompress(input: &[u8]) -> Result<Vec<u8>, snap::Error> {
        let mut decoder = snap::raw::Decoder::new();
        decoder.decompress_vec(input)
    }

    pub fn handle_err(err: snap::Error) -> super::JsError {
        super::JsError::new(err.to_string().as_str())
    }
}
