use oak_source_map::*;

#[test]
fn test_builder_basic() {
    let mut builder = SourceMapBuilder::new();
    let source_idx = builder.add_source("test.ts");
    builder.add_mapping(0, 0, Some(source_idx), Some(0), Some(0), None);

    let sm = builder.build();
    assert_eq!(sm.version, 3);
    assert_eq!(sm.sources.len(), 1);
    assert!(!sm.mappings.is_empty());
}

#[test]
fn test_builder_multiple_mappings() {
    let mut builder = SourceMapBuilder::new();
    let source_idx = builder.add_source("test.ts");

    builder.add_mapping(0, 0, Some(source_idx), Some(0), Some(0), None);
    builder.add_mapping(0, 10, Some(source_idx), Some(0), Some(10), None);
    builder.add_mapping(1, 0, Some(source_idx), Some(1), Some(0), None);

    let sm = builder.build();
    let mappings = sm.parse_mappings().unwrap();
    assert_eq!(mappings.len(), 3);
}

#[test]
fn test_builder_with_names() {
    let mut builder = SourceMapBuilder::new();
    let source_idx = builder.add_source("test.ts");
    let name_idx = builder.add_name("foo");

    builder.add_mapping(0, 0, Some(source_idx), Some(0), Some(0), Some(name_idx));

    let sm = builder.build();
    assert_eq!(sm.names.len(), 1);
    assert_eq!(sm.names[0], "foo");
}

#[test]
fn test_builder_source_content() {
    let mut builder = SourceMapBuilder::new();
    let source_idx = builder.add_source("test.ts");
    builder.set_source_content(source_idx, "const x = 1;");

    let sm = builder.build();
    assert_eq!(sm.sources_content.len(), 1);
    assert_eq!(sm.sources_content[0], Some("const x = 1;".to_string()));
}

#[test]
fn test_compose_empty() {
    let composer = SourceMapComposer::new();
    let result = composer.compose().unwrap();
    assert_eq!(result.version, 3);
}

#[test]
fn test_compose_single() {
    let mut sm = SourceMap::new();
    sm.add_source("test.ts");

    let composer = SourceMapComposer::new().add(sm.clone());
    let result = composer.compose().unwrap();

    assert_eq!(result.sources, sm.sources);
}

#[test]
fn test_decoder_lookup() {
    let json = r#"{"version":3,"sources":["a.js"],"names":[],"mappings":"AAAA,SAASA"}"#;
    let sm = SourceMap::parse(json).unwrap();
    let decoder = SourceMapDecoder::new(sm).unwrap();

    let pos = decoder.lookup(0, 0);
    assert!(pos.is_some());
}

#[test]
fn test_decoder_lookup_full() {
    let json = r#"{"version":3,"sources":["a.js"],"names":["foo"],"mappings":"AAAA"}"#;
    let sm = SourceMap::parse(json).unwrap();
    let decoder = SourceMapDecoder::new(sm).unwrap();

    let pos = decoder.lookup_full(0, 0);
    assert!(pos.is_some());
}

#[test]
fn test_mapping_ordering() {
    let m1 = Mapping::generated_only(0, 5);
    let m2 = Mapping::generated_only(0, 10);
    let m3 = Mapping::generated_only(1, 0);

    assert!(m1 < m2);
    assert!(m2 < m3);
}

#[test]
fn test_mapping_has_source() {
    let m1 = Mapping::generated_only(0, 0);
    assert!(!m1.has_source());

    let m2 = Mapping::full(0, 0, 0, 0, 0, None);
    assert!(m2.has_source());
}

#[test]
fn test_bounded_mapping() {
    let mapping = Mapping::generated_only(0, 5);
    let bounded = BoundedMapping::new(mapping, 5, 10);

    assert!(bounded.contains_column(5));
    assert!(bounded.contains_column(7));
    assert!(!bounded.contains_column(10));
    assert!(!bounded.contains_column(4));
}

#[test]
fn test_parse_minimal() {
    let json = r#"{"version":3,"sources":[],"names":[],"mappings":""}"#;
    let sm = SourceMap::parse(json).unwrap();
    assert_eq!(sm.version, 3);
    assert!(sm.sources.is_empty());
}

#[test]
fn test_parse_with_sources() {
    let json = r#"{"version":3,"sources":["foo.js","bar.js"],"names":[],"mappings":"AAAA"}"#;
    let sm = SourceMap::parse(json).unwrap();
    assert_eq!(sm.sources.len(), 2);
    assert_eq!(sm.sources[0], "foo.js");
    assert_eq!(sm.sources[1], "bar.js");
}

#[test]
fn test_invalid_version() {
    let json = r#"{"version":2,"sources":[],"names":[],"mappings":""}"#;
    let result = SourceMap::parse(json);
    assert!(matches!(result, Err(SourceMapError::InvalidVersion(2))));
}

#[test]
fn test_add_source() {
    let mut sm = SourceMap::new();
    let idx = sm.add_source("test.js");
    assert_eq!(idx, 0);
    assert_eq!(sm.sources.len(), 1);

    let idx2 = sm.add_source("test.js");
    assert_eq!(idx2, 0);
    assert_eq!(sm.sources.len(), 1);
}

#[test]
fn test_to_json() {
    let mut sm = SourceMap::new();
    sm.add_source("test.js");
    let json = sm.to_json().unwrap();
    assert!(json.contains("\"version\":3"));
    assert!(json.contains("\"sources\":[\"test.js\"]"));
}

#[test]
fn test_parse_mappings() {
    let json = r#"{"version":3,"sources":["a.js"],"names":[],"mappings":"AAAA;ACDA"}"#;
    let sm = SourceMap::parse(json).unwrap();
    let mappings = sm.parse_mappings().unwrap();
    assert_eq!(mappings.len(), 2);
}

#[test]
fn test_encode_zero() {
    assert_eq!(vlq_encode(0), "A");
}

#[test]
fn test_encode_positive() {
    assert_eq!(vlq_encode(1), "C");
    assert_eq!(vlq_encode(2), "E");
    assert_eq!(vlq_encode(15), "e");
    assert_eq!(vlq_encode(16), "gB");
    assert_eq!(vlq_encode(31), "+B");
    assert_eq!(vlq_encode(32), "gC");
}

#[test]
fn test_encode_negative() {
    assert_eq!(vlq_encode(-1), "D");
    assert_eq!(vlq_encode(-2), "F");
    assert_eq!(vlq_encode(-16), "hB");
}

#[test]
fn test_decode_zero() {
    let result = vlq_decode("A").unwrap();
    assert_eq!(result, (0, 1));
}

#[test]
fn test_decode_positive() {
    let result1 = vlq_decode("C").unwrap();
    assert_eq!(result1, (1, 1));
    let result2 = vlq_decode("gB").unwrap();
    assert_eq!(result2, (16, 2));
}

#[test]
fn test_decode_negative() {
    let result1 = vlq_decode("D").unwrap();
    assert_eq!(result1, (-1, 1));
    let result2 = vlq_decode("hB").unwrap();
    assert_eq!(result2, (-16, 2));
}

#[test]
fn test_roundtrip() {
    for value in [-1000, -100, -10, -1, 0, 1, 10, 100, 1000, 12345] {
        let encoded = vlq_encode(value);
        let (decoded, _) = vlq_decode(&encoded).unwrap();
        assert_eq!(decoded, value, "Failed for value {}", value);
    }
}

#[test]
fn test_decode_many() {
    let encoded = "AAAA";
    let decoded = vlq_decode_many(encoded).unwrap();
    assert_eq!(decoded, vec![0, 0, 0, 0]);
}

#[test]
fn test_encode_many() {
    let values = vec![0, 1, 2, 3];
    let encoded = vlq_encode_many(&values);
    assert_eq!(encoded, "ACEG");
}
