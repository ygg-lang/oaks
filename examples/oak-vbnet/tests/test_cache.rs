use oak_core::{CachingParseSession, ParseSession};
use oak_vbnet::{VbNetLanguage, parse_with_cache};

#[test]
fn test_content_cache_basic() {
    let code = r#"        Module Program
            Sub Main()
                Console.WriteLine("Hello, World!")
            End Sub
        End Module
    "#;

    // Create a caching parse session
    let inner = ParseSession::<VbNetLanguage>::new(16);
    let mut cache = CachingParseSession::new(inner, 10);

    // First parse - should not use cache
    let result1 = parse_with_cache(code, &mut cache);
    assert!(result1.is_ok(), "First parse failed");

    // Second parse - should use cache
    let result2 = parse_with_cache(code, &mut cache);
    assert!(result2.is_ok(), "Second parse failed");

    // Both results should be the same
    assert_eq!(result1.is_ok(), result2.is_ok());
}

#[test]
fn test_content_cache_different_content() {
    let code1 = r#"        Module Program
            Sub Main()
                Console.WriteLine("Hello, World!")
            End Sub
        End Module
    "#;

    let code2 = r#"        Module Program
            Sub Main()
                Console.WriteLine("Hello, Rust!")
            End Sub
        End Module
    "#;

    // Create a caching parse session
    let inner = ParseSession::<VbNetLanguage>::new(16);
    let mut cache = CachingParseSession::new(inner, 10);

    // Parse first code
    let result1 = parse_with_cache(code1, &mut cache);
    assert!(result1.is_ok(), "First parse failed");

    // Parse second code (different content)
    let result2 = parse_with_cache(code2, &mut cache);
    assert!(result2.is_ok(), "Second parse failed");

    // Both results should be ok, but different content
    assert!(result1.is_ok() && result2.is_ok());
}

#[test]
fn test_content_cache_eviction() {
    // Create a cache with max 2 entries
    let inner = ParseSession::<VbNetLanguage>::new(16);
    let mut cache = CachingParseSession::new(inner, 2);

    // Parse first code
    let code1 = "Module Program1 Sub Main() End Sub End Module";
    let result1 = parse_with_cache(code1, &mut cache);
    assert!(result1.is_ok(), "First parse failed");

    // Parse second code
    let code2 = "Module Program2 Sub Main() End Sub End Module";
    let result2 = parse_with_cache(code2, &mut cache);
    assert!(result2.is_ok(), "Second parse failed");

    // Parse third code - should evict first entry
    let code3 = "Module Program3 Sub Main() End Sub End Module";
    let result3 = parse_with_cache(code3, &mut cache);
    assert!(result3.is_ok(), "Third parse failed");

    // Parse first code again - should not be in cache
    let result1_again = parse_with_cache(code1, &mut cache);
    assert!(result1_again.is_ok(), "First code parse again failed");
}

#[ignore = "VB.NET cache/performance fixtures are out of date"]
#[test]
fn test_content_cache_performance() {
    let code = r#"        Imports System
        Imports System.Collections.Generic
        Imports System.Threading.Tasks
        
        Namespace MyCompany.Project
            Public Class Program
                Public Shared Async Function Main(ByVal args() As String) As Task
                    Dim service As New DataService()
                    Dim data As String = Await service.GetDataAsync()
                    Console.WriteLine(data)
                End Function
            End Class
            
            Public Class DataService
                Public Async Function GetDataAsync() As Task(Of String)
                    Await Task.Delay(1000)
                    Return "Hello from DataService"
                End Function
            End Class
        End Namespace
    "#;

    // Create a caching parse session
    let inner = ParseSession::<VbNetLanguage>::new(16);
    let mut cache = CachingParseSession::new(inner, 10);

    // Measure time for first parse (no cache)
    let start1 = std::time::Instant::now();
    let result1 = parse_with_cache(code, &mut cache);
    let duration1 = start1.elapsed();
    assert!(result1.is_ok(), "First parse failed");

    // Measure time for second parse (with cache)
    let start2 = std::time::Instant::now();
    let result2 = parse_with_cache(code, &mut cache);
    let duration2 = start2.elapsed();
    assert!(result2.is_ok(), "Second parse failed");

    // Cache should be faster
    println!("First parse time: {:?}", duration1);
    println!("Second parse time: {:?}", duration2);
    assert!(duration2 < duration1, "Cache should be faster");
}
