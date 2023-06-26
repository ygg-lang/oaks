use oak_core::{CachingParseSession, ParseSession};
use oak_vbnet::{VbNetLanguage, parse, parse_with_cache};

fn main() {
    // Test code
    let code = r#"        Module Program
            Sub Main()
                Console.WriteLine("Hello, World!")
            End Sub
        End Module
    "#;

    println!("Testing parser cache performance...");
    println!("Code length: {} characters", code.len());
    println!();

    // Test without cache
    println!("1. Testing without cache:");
    let start1 = std::time::Instant::now();
    for _ in 0..10 {
        let result = parse(code);
        assert!(result.is_ok(), "Parse failed");
    }
    let duration1 = start1.elapsed();
    println!("Time for 10 parses without cache: {:?}", duration1);
    println!();

    // Test with cache
    println!("2. Testing with cache:");
    let inner = ParseSession::<VbNetLanguage>::new(16);
    let mut cache = CachingParseSession::new(inner, 10);

    let start2 = std::time::Instant::now();
    for _ in 0..10 {
        let result = parse_with_cache(code, &mut cache);
        assert!(result.is_ok(), "Parse with cache failed");
    }
    let duration2 = start2.elapsed();
    println!("Time for 10 parses with cache: {:?}", duration2);
    println!();

    // Calculate speedup
    let speedup = duration1.as_secs_f64() / duration2.as_secs_f64();
    println!("3. Performance comparison:");
    println!("Speedup: {:.2}x faster with cache", speedup);
    println!("Cache size: {} entries", cache.content_cache().len());
    println!();

    println!("Test completed successfully!");
}
