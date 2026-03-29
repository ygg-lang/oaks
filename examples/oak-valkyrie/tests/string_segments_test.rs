use oak_valkyrie::{
    ast::{Identifier, Span, StringSegment, TermExpression},
    parser::parse_string_segments,
};

#[test]
fn test_plain_text() {
    let segments = parse_string_segments("hello world", 0, false);
    assert_eq!(segments.len(), 1);
    match &segments[0] {
        StringSegment::Text { content, .. } => assert_eq!(content, "hello world"),
        _ => panic!("Expected Text segment"),
    }
}

#[test]
fn test_simple_interpolation() {
    let segments = parse_string_segments("hello {name}!", 0, false);
    assert_eq!(segments.len(), 3);

    match &segments[0] {
        StringSegment::Text { content, .. } => assert_eq!(content, "hello "),
        _ => panic!("Expected Text segment"),
    }

    match &segments[1] {
        StringSegment::Interpolation { expr, is_fluent, .. } => {
            assert!(!is_fluent);
            match expr.as_ref() {
                TermExpression::Identifier(ident) => assert_eq!(ident.name, "name"),
                _ => panic!("Expected Identifier"),
            }
        }
        _ => panic!("Expected Interpolation segment"),
    }

    match &segments[2] {
        StringSegment::Text { content, .. } => assert_eq!(content, "!"),
        _ => panic!("Expected Text segment"),
    }
}

#[test]
fn test_escaped_braces() {
    let segments = parse_string_segments(r"hello \{world\}", 0, false);
    assert_eq!(segments.len(), 1);
    match &segments[0] {
        StringSegment::Text { content, .. } => assert_eq!(content, "hello {world}"),
        _ => panic!("Expected Text segment"),
    }
}

#[test]
fn test_raw_string() {
    let segments = parse_string_segments("hello {name}!", 0, true);
    assert_eq!(segments.len(), 1);
    match &segments[0] {
        StringSegment::Text { content, .. } => assert_eq!(content, "hello {name}!"),
        _ => panic!("Expected Text segment"),
    }
}

#[test]
fn test_fluent_marker() {
    let segments = parse_string_segments("hello {\u{07DF}name}!", 0, false);
    assert_eq!(segments.len(), 3);

    match &segments[1] {
        StringSegment::Interpolation { is_fluent, .. } => {
            assert!(is_fluent);
        }
        _ => panic!("Expected Interpolation segment"),
    }
}

#[test]
fn test_nested_braces() {
    let segments = parse_string_segments("hello {foo{bar}}!", 0, false);
    assert_eq!(segments.len(), 3);

    match &segments[1] {
        StringSegment::Interpolation { expr, .. } => match expr.as_ref() {
            TermExpression::Identifier(ident) => assert_eq!(ident.name, "foo{bar}"),
            _ => panic!("Expected Identifier"),
        },
        _ => panic!("Expected Interpolation segment"),
    }
}

#[test]
fn test_multiple_interpolations() {
    let segments = parse_string_segments("{a}{b}{c}", 0, false);
    assert_eq!(segments.len(), 3);

    for (i, segment) in segments.iter().enumerate() {
        match segment {
            StringSegment::Interpolation { expr, .. } => match expr.as_ref() {
                TermExpression::Identifier(ident) => {
                    let expected = match i {
                        0 => "a",
                        1 => "b",
                        2 => "c",
                        _ => unreachable!(),
                    };
                    assert_eq!(ident.name, expected);
                }
                _ => panic!("Expected Identifier"),
            },
            _ => panic!("Expected Interpolation segment"),
        }
    }
}
