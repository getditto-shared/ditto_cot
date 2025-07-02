#![no_main]
use libfuzzer_sys::fuzz_target;
use ditto_cot::xml_parser::parse_cot;

fuzz_target!(|data: &str| {
    let _ = parse_cot(data);
});