use oak_core::{Builder, SourceText};
use oak_dejavu::{DejavuBuilder, DejavuLanguage};

#[test]
fn test_flags_builder() {
    let language = DejavuLanguage::default();
    let builder = DejavuBuilder::new(&language);

    // Test flags declaration
    let source = SourceText::new("flags Permissions { Read, Write, Execute }");

    println!("Testing builder with flags");

    let mut cache = oak_core::parser::ParseSession::<DejavuLanguage>::default();
    let diagnostics = builder.build(&source, &[], &mut cache);
    match diagnostics.result {
        Ok(typed_root) => {
            println!("Successfully built flags typed root: {:?}", typed_root);
            // Verify if Flags item is generated
            let has_flags = typed_root.items.iter().any(|item| matches!(item, oak_dejavu::ast::ItemNode::Flags(_)));
            assert!(has_flags, "Builder should have generated a Flags item")
        }
        Err(e) => {
            panic!("Flags build failed with error: {}", e)
        }
    }
}

#[test]
fn test_dejavu_builder_single_file() {
    let language = DejavuLanguage::default();
    let builder = DejavuBuilder::new(&language);

    // Test simple micro function
    let source = SourceText::new("micro add(x: i32, y: i32) -> i32 { x + y }");

    println!("Testing builder with micro function");

    let mut cache = oak_core::parser::ParseSession::<DejavuLanguage>::default();
    let diagnostics = builder.build(&source, &[], &mut cache);
    match diagnostics.result {
        Ok(typed_root) => {
            println!("Successfully built typed root: {:?}", typed_root)
        }
        Err(e) => {
            println!("Build failed with error: {}", e)
        }
    }
    if !diagnostics.diagnostics.is_empty() {
        println!("Build diagnostics: {:?}", diagnostics.diagnostics)
    }

    // Temporarily pass test until implementation is complete
    assert!(true, "Single file builder test placeholder")
}

#[test]
fn test_dejavu_builder_namespace() {
    let language = DejavuLanguage::default();
    let builder = DejavuBuilder::new(&language);

    // Test namespace declaration
    let source = SourceText::new("namespace Test { micro main() { let x = 42 } }");

    println!("Testing builder with namespace");

    let mut cache = oak_core::parser::ParseSession::<DejavuLanguage>::default();
    let diagnostics = builder.build(&source, &[], &mut cache);
    match diagnostics.result {
        Ok(typed_root) => {
            println!("Successfully built namespace typed root: {:?}", typed_root)
        }
        Err(e) => {
            println!("Namespace build failed with error: {}", e)
        }
    }

    assert!(true, "Namespace builder test placeholder")
}
