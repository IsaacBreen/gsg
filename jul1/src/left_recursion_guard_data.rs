#[derive(Debug, Clone, PartialEq, Default)]
pub struct LeftRecursionGuardData {
    to_pass: Option<usize>,
    to_skip: Vec<usize>,
}
