use oak_glob::parse;

#[ignore = "Glob parser still returns Not implemented"]
#[test]
fn test_parse_glob() {
    let root = parse("*.txt").expect("glob should parse");
    let _ = root;
}
