use oak_ignore::{IgnoreParser, register};

#[test]
fn test_parse_ignore() {
    let input = "# This is a comment\n*.txt\n!important.txt\n";
    let parser = IgnoreParser;
    let tokens = parser.parse(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, "pattern");
    assert_eq!(tokens[0].value, "*.txt");
    assert_eq!(tokens[1].kind, "pattern");
    assert_eq!(tokens[1].value, "!important.txt");
}
