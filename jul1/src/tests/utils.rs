#[macro_export]
macro_rules! assert_parses {
    ($combinator:expr, $input:expr) => {{
        println!("beginning assert_parses");
        let (mut parser, _, _) = $combinator.parser($crate::RightData::default());
        println!("constructed parser");
        println!("Stats: {:?}", parser.stats());
        let mut result = Ok(());
        for &byte in $input.as_bytes() {
            let (right_data, up_data) = parser.step(byte);
            println!("Stats: {:?}", parser.stats());
            if right_data.is_empty() && up_data.is_empty() {
                result = Err(format!("Parser failed at byte: {}", byte as char));
                break;
            }
        }
        assert!(result.is_ok(), "{}", result.err().unwrap_or_default());
    }};
}