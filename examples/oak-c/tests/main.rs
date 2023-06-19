use oak_c::{CLanguage, lexer::CLexer};
use oak_core::{Lexer, source::SourceText};

fn test_c_lexer() {
    println!("Testing C Lexer...");

    let language = CLanguage;
    let lexer = CLexer::new(&language);

    let test_code = r#"
#include <stdio.h>

int main() {
    int x = 42;
    float y = 3.14;
    char c = 'a';
    char *str = "Hello, World!";
    
    if (x > 0) {
        printf("%s\n", str);
    }
    
    return 0;
}
"#;

    let source = SourceText::new(test_code);
    let mut builder = oak_core::GreenBuilder::new(1024);
    let cache = oak_core::IncrementalCache::new(&mut builder);
    let output = lexer.lex_incremental(&source, 0, cache);

    match output.result {
        Ok(tokens) => {
            println!("Tokens found: {}", tokens.len());

            // 显示前几个 tokens 作为示例
            for (i, token) in tokens.iter().take(20).enumerate() {
                println!("  {}: {:?}", i, token);
            }
        }
        Err(e) => {
            println!("Error during lexing: {:?}", e);
        }
    }

    println!("C Lexer test completed successfully!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        test_c_lexer();
    }
}

fn main() {
    test_c_lexer();
}
