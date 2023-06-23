use oak_core::errors::OakError;
use oak_svelte::{SvelteLanguage, SvelteLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_svelte_lexer() -> Result<(), OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(SvelteLanguage::default()));
    let lexer = SvelteLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("svelte").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)?;
    Ok(())
}
