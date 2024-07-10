#[derive(Debug, Clone, PartialEq)]
pub struct LeftRecursionGuardData {
    to_pass: Option<usize>,
    to_skip: Vec<usize>,
}
