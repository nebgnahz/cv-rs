extern crate cv;
mod utils;

use cv::videoio::*;

#[test]
fn test_ayuv() {
    test_4cc("AYUV", 0x56555941);
}

#[test]
fn test_cljr() {
    test_4cc("CLJR", 0x524A4C43);
}

#[test]
fn test_uyvp() {
    test_4cc("UYVP", 0x50565955);
}

#[test]
fn test_vyuy() {
    test_4cc("VYUY", 0x59555956);
}

fn test_4cc(string: &'static str, integer: u32) {
    let integer_value = codec_name_from_4cc(string).unwrap();
    let string_value = codec_name_to_4cc(integer);
    assert_eq!(string_value, string);
    assert_eq!(integer_value, integer);
}
