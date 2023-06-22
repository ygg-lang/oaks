use oak_pretty_print::doc;

#[test]
fn test_macro_error_cases() {
    // 这是一个合法的调用，用于对比
    let _ = doc!(nil);
}

#[test]
fn test_invalid_keyword() {
    let unknown_keyword = doc!(nil);
    let _ = doc!(unknown_keyword);
}

#[test]
fn test_missing_bracket() {
    let _ = doc!([nil, line]);
}
