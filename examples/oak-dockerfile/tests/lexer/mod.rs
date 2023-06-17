use oak_core::helpers::LexerTester;
use oak_docker_file::{DockerFileLanguage, DockerFileLexer};
use std::path::Path;

#[test]
fn test_docker_file_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = DockerFileLexer::new(&DockerFileLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("dockerfile");
    match test_runner.run_tests::<DockerFileLanguage, _>(lexer) {
        Ok(()) => println!("DockerFile lexer tests passed!"),
        Err(e) => panic!("DockerFile lexer tests failed: {}", e),
    }
}