use ad_event_log_parser::{parse_date, parse_ip, parse_price, parse_time, parse_uuid};


#[test]
fn test_parse_date() {
    let input = "06-11-2024";
    assert!(parse_date(input).is_ok());
}

#[test]
fn test_parse_time() {
    let input = "11:46:26";
    assert!(parse_time(input).is_ok());
}

#[test]
fn test_parse_uuid() {
    let input = "1fed5c63-5a48-4568-9220-30612b041a3d";
    assert!(parse_uuid(input).is_ok());
}

#[test]
fn test_parse_ip() {
    let input = "103.170.177.248";
    assert!(parse_ip(input).is_ok());
}

#[test]
fn test_parse_price() {
    let input = "0.000001";
    assert!(parse_price(input).is_ok());
}
