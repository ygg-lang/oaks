use oak_core::{ParseSession, Parser, SourceText};
use oak_vue::{VueLanguage, VueParser};

#[test]
fn test_vue_parser() {
    let source = SourceText::new(
        r#"<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
  </div>
</template>

<script setup>
const msg = 'Hello Vue 3!'
</script>

<style scoped>
.hello {
  color: #42b983;
}
</style>"#,
    );
    let language = VueLanguage::default();
    let parser = VueParser::new(&language);
    let mut session = ParseSession::default();

    println!("Starting parse...");
    let result = parser.parse(&source, &[], &mut session);
    println!("Parse completed!");

    if !result.diagnostics.is_empty() {
        println!("Diagnostics:");
        for diag in &result.diagnostics {
            println!("{:?}", diag);
        }
    }

    assert!(result.diagnostics.is_empty(), "Parser should not return errors");
    println!("Test passed!");
}
