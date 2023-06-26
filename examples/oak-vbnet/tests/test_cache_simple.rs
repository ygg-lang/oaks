use oak_core::{CachingParseSession, ParseSession, SourceText};
use oak_vbnet::{VbNetBuilder, VbNetLanguage, VbNetLexer, VbNetParser};

fn main() {
    // Test code
    let code = r#"Module Program
    Sub Main()
        Console.WriteLine("Hello, World!")
    End Sub
End Module"#;

    println!("Testing parser cache...");
    println!();

    // Create language and builder
    let language = VbNetLanguage::new();
    let builder = VbNetBuilder::new(&language);
    let source = SourceText::new(code);

    // Create caching parse session
    let inner = ParseSession::<VbNetLanguage>::new(16);
    let mut cache = CachingParseSession::new(inner, 10);

    // First parse
    println!("1. First parse (no cache hit):");
    let start1 = std::time::Instant::now();
    let result1 = builder.build(&source, &[], &mut cache);
    let duration1 = start1.elapsed();
    println!("   Time: {:?}", duration1);
    println!("   Result: {:?}", result1.result.is_ok());
    println!();

    // Second parse (should hit cache)
    println!("2. Second parse (cache hit):");
    let start2 = std::time::Instant::now();
    let result2 = builder.build(&source, &[], &mut cache);
    let duration2 = start2.elapsed();
    println!("   Time: {:?}", duration2);
    println!("   Result: {:?}", result2.result.is_ok());
    println!();

    // Check cache size
    println!("3. Cache status:");
    println!("   Cache entries: {}", cache.content_cache().len());
    println!();

    // Performance comparison
    println!("4. Performance comparison:");
    println!("   First parse: {:?}", duration1);
    println!("   Second parse: {:?}", duration2);
    if duration2 < duration1 {
        let speedup = duration1.as_secs_f64() / duration2.as_secs_f64();
        println!("   Speedup: {:.2}x faster with cache", speedup);
    }
    else {
        println!("   No speedup observed");
    }

    println!("\nTest completed!");
}
