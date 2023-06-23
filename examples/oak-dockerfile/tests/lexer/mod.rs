use oak_dockerfile::{DockerfileLanguage, DockerfileLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_dockerfile_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(DockerfileLanguage::default()));
    let lexer = DockerfileLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("dockerfile").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<DockerfileLanguage, _>(&lexer) {
        Ok(()) => println!("Dockerfile lexer tests passed!"),
        Err(e) => panic!("Dockerfile lexer tests failed: {}", e),
    }
}

#[test]
fn ready() {
    // 简单的就绪测试
}
