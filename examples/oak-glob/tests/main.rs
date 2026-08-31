use oak_glob::parse;

#[test]
fn test_parse_glob() {
    let root = parse("*.txt").expect("glob should parse");
    let _ = root;
}
