//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use snappy;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const TEST_STRING: &str = "Hello Hello Hello Hello Hello Hello ";
const TEST_STRING_COMPRESSED: &[u8] = b"$\x14Hello v\x06\x00";

#[wasm_bindgen_test]
fn test_compress() {
    let compressed = snappy::compress(TEST_STRING);
    assert!(compressed.is_ok(), "Compressed some bytes")
}

#[wasm_bindgen_test]
fn test_compress_raw() {
    let compressed = snappy::compress_raw(TEST_STRING.as_bytes());
    assert!(compressed.is_ok(), "Compressed some bytes");
    assert_eq!(*compressed.unwrap_or(Box::new([])), *TEST_STRING_COMPRESSED)
}

#[wasm_bindgen_test]
fn test_decompress() {
    let compressed = snappy::compress(TEST_STRING).unwrap_or(Box::new([]));
    let decompressed = snappy::decompress(compressed.as_ref());
    assert!(decompressed.is_ok(), "Decompressed some bytes");
    assert_eq!(
        decompressed.unwrap_or(String::new()),
        TEST_STRING,
        "Decompressed original string"
    );
}

#[wasm_bindgen_test]
fn test_decompress_raw() {
    let compressed = snappy::compress_raw(TEST_STRING.as_bytes()).unwrap_or(Box::new([]));
    let decompressed = snappy::decompress_raw(compressed.as_ref());
    assert!(decompressed.is_ok(), "Decompressed some bytes");
    assert_eq!(
        decompressed.unwrap_or(Box::new([])).as_ref(),
        TEST_STRING.as_bytes(),
        "Decompressed original string"
    );
}
