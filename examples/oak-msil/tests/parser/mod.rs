// use oak_core::helpers::LexerTester;
// use oak_msil::{MsilLanguage, MsilParser};
// use std::{path::Path, time::Duration};
//
// #[test]
// fn test_msil_parser() {
//     let here = Path::new(env!("CARGO_MANIFEST_DIR"));
//     let language = Box::leak(Box::new(MsilLanguage::default()));
//     let lexer = MsilParser::new(language);
//     let test_runner = LexerTester::new(here.join("tests/files")).with_extension("il").with_timeout(Duration::from_secs(5));
//     match test_runner.run_tests::<MsilLanguage, _>(lexer) {
//         Ok(()) => println!("Msil files tests passed!"),
//         Err(e) => panic!("Msil files tests failed: {}", e),
//     }
// }
