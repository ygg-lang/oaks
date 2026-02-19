// use oak_java::{JavaLanguage, JavaParser};
// use oak_core::{Parser, source::SourceText};
//
// #[test]
// fn test_generics() {
//     let code = r#"
//     public class GenericClass<T extends Number & Comparable<T>> {
//         public <U> void method(U value) {
//             List<String> list = new ArrayList<>();
//         }
//     }
//     "#;
//
//     let language = JavaLanguage::default();
//     let parser = JavaParser::new(&language);
//     let mut cache = oak_core::parser::ParseSession::new();
//
//     let result = parser.parse(code, &[], &mut cache);
//     println!("Generics test result: {:?}", result);
//     assert!(result.diagnostics.is_empty(), "Parsing generics failed");
// }
//
// #[test]
// fn test_lambda_expressions() {
//     let code = r#"
//     public class LambdaTest {
//         public void test() {
//             Runnable r = () -> System.out.println("Hello");
//             Function<Integer, String> f = (x) -> String.valueOf(x);
//             BiFunction<Integer, String, Boolean> bf = (x, y) -> x > y.length();
//             Supplier<String> s = () -> {
//                 return "Hello World";
//             };
//         }
//     }
//     "#;
//
//     let language = JavaLanguage::default();
//     let parser = JavaParser::new(&language);
//     let mut cache = oak_core::parser::ParseSession::new();
//
//     let result = parser.parse(code, &[], &mut cache);
//     println!("Lambda test result: {:?}", result);
//     assert!(result.diagnostics.is_empty(), "Parsing lambdas failed");
// }
//
// #[test]
// fn test_annotation_type() {
//     let code = r#"
//     public @interface MyAnnotation {
//         String value() default "default";
//         int number() default 42;
//         String[] values() default {"a", "b"};
//     }
//
//     @MyAnnotation(value = "test", number = 100)
//     public class AnnotatedClass {
//     }
//     "#;
//
//     let language = JavaLanguage::default();
//     let parser = JavaParser::new(&language);
//     let mut cache = oak_core::parser::ParseSession::new();
//
//     let result = parser.parse(code, &[], &mut cache);
//     println!("Annotation test result: {:?}", result);
//     assert!(result.diagnostics.is_empty(), "Parsing annotations failed");
// }
//
// #[test]
// fn test_enum_enhancements() {
//     let code = r#"
//     public enum Color {
//         RED(255, 0, 0),
//         GREEN(0, 255, 0),
//         BLUE(0, 0, 255);
//
//         private final int r;
//         private final int g;
//         private final int b;
//
//         Color(int r, int g, int b) {
//             this.r = r;
//             this.g = g;
//             this.b = b;
//         }
//
//         public int getR() {
//             return r;
//         }
//     }
//     "#;
//
//     let language = JavaLanguage::default();
//     let parser = JavaParser::new(&language);
//     let mut cache = oak_core::parser::ParseSession::new();
//
//     let result = parser.parse(code, &[], &mut cache);
//     println!("Enum test result: {:?}", result);
//     assert!(result.diagnostics.is_empty(), "Parsing enum enhancements failed");
// }
