use dejavu_ast::{DejavuTemplate, Pattern};

#[test]
fn test_template_creation() {
    let template = DejavuTemplate::new();
    assert!(template.is_dyn());
    assert!(!template.is_aot());
}

#[test]
fn test_pattern_bound_variables() {
    let pattern = Pattern::Tuple(vec![Pattern::Identifier("x".to_string()), Pattern::Identifier("y".to_string())]);
    let vars = pattern.bound_variables();
    assert_eq!(vars, vec!["x", "y"]);
}
