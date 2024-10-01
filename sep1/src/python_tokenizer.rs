struct PythonSpecialTokenizer {
    scope_count: usize,
    indents: Vec<Vec<u8>>,
}

impl Tokenizer for