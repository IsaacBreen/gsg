#[macro_export]
macro_rules! assert_parses {
    ($combinator:expr, $input:expr) => {{
        let (mut parser, _, _) = $combinator.parser(HorizontalData::default());
        let mut result = Ok(());
        for &byte in $input.as_bytes() {
            let (horizontal_data, vertical_data) = parser.step(byte);
            if horizontal_data.is_empty() && vertical_data.is_empty() {
                result = Err(format!("Parser failed at byte: {}", byte as char));
                break;
            }
        }
        assert!(result.is_ok(), "{}", result.err().unwrap_or_default());
    }};
}