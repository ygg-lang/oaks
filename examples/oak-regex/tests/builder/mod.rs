use oak_regex::{RegexLanguage, RegexParser};

#[test]
fn test_regex_builder() {
    let language = RegexLanguage::default();
    let _parser = RegexParser::new(&language);

    // TODO: Implement actual builder tests once build_incremental is fully implemented
    let _source = r"[a-z]+";

    // For now, just test that the builder can be created
    assert!(true);
}
