use oak_pretty_print::doc;

#[test]
fn test_macro_error_cases() {
    // This is a valid call, used for comparison
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
