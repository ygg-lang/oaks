#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_basic_smalltalk() {
        let content = fs::read_to_string("tests/lexer/basic.st").expect("Failed to read basic.st");

        // Basic test to ensure file can be read
        assert!(!content.is_empty());
        assert!(content.contains("Smalltalk"));
    }
}
