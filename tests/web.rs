//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use snappy_wasm;

wasm_bindgen_test_configure!(run_in_browser);

const TEST_STRING: &str = "Hello World Hello World Hello World Hello World Hello World Hello World Hello World Hello World Hello World";

#[wasm_bindgen_test]
fn test_compress() {
    let compressed = snappy_wasm::compress(TEST_STRING.as_bytes());
    assert!(compressed.is_some(), "Compressed some bytes")
}

#[wasm_bindgen_test]
fn test_compress_str() {
    let compressed = snappy_wasm::compress_str(TEST_STRING);
    assert!(compressed.is_some(), "Compressed some bytes")
}

#[wasm_bindgen_test]
fn test_decompress() {
    let compressed = snappy_wasm::compress(TEST_STRING.as_bytes()).unwrap();
    let decompressed = snappy_wasm::decompress(compressed.as_ref());
    assert!(decompressed.is_some(), "Decompressed some bytes");
    assert_eq!(decompressed.unwrap().as_ref(), TEST_STRING.as_bytes(), "Decompressed original string");
}

#[wasm_bindgen_test]
fn test_decompress_str() {
    let compressed = snappy_wasm::compress_str(TEST_STRING).unwrap();
    let decompressed = snappy_wasm::decompress_str(compressed.as_ref());
    assert!(decompressed.is_ok(), "Decompressed some bytes");
    assert_eq!(decompressed.unwrap(), TEST_STRING, "Decompressed original string");
}
