extern crate cv;
mod utils;

use cv::videoio::*;

#[test]
fn test_4cc() {
    let code = "abcd";
    let integer_value = codec_name_from_4cc(code).unwrap();
    let string_value = codec_name_to_4cc(integer_value);
    assert_eq!(1684234849, integer_value);
    assert_eq!(string_value, code);
}
