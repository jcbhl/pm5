use std::time::Duration;

pub fn decode_to_time(lo: u8, mid: u8, hi: u8) -> Duration {
    let hundreths_of_secs = u32::from_le_bytes([lo, mid, hi, 0x00]);
    Duration::from_millis((hundreths_of_secs as u64) * 10)
}

// returns TENTHS of meters
pub fn decode_to_distance(lo: u8, mid: u8, hi: u8) -> u32 {
    // TODO this may round
    u32::from_le_bytes([lo, mid, hi, 0x00])
}

// --- tests ---

#[test]
fn test_basic_time() {
    let res = decode_to_time(0x5f, 0x06, 0x00);
    assert_eq!(res.as_secs(), 16);
}

#[test]
fn test_1min_time() {
    let res = decode_to_time(0x70, 0x17, 0x00);
    assert_eq!(res.as_secs(), 60);
    assert_eq!(res.subsec_millis(), 0);
}

#[test]
fn test_basic_meters() {
    let res = decode_to_distance(0x5c, 0x01, 0x00);
    assert_eq!(res, 348);
}

// 0x0031 response
//           time    dist    workout type   interval type   workout state  rowing state    stroke state    total distance   workout duration   duration type   drag factor
//          5F 6 0 | 5C 1 0 |  1           |  1           |  1            |  0           | 1             | 0 0 0          |  0 0 0           | 80            | 6A
