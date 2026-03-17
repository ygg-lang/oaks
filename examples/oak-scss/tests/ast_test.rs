use oak_core::{Builder, source::StringSource};
use oak_scss::{ast::ScssVisitor, builder::ScssBuilder, language::ScssLanguage};

/// Test visitor that counts nodes
struct NodeCounter {
    rule_sets: usize,
    mixins: usize,
    functions: usize,
    variables: usize,
}

impl NodeCounter {
    fn new() -> Self {
        Self { rule_sets: 0, mixins: 0, functions: 0, variables: 0 }
    }
}

impl ScssVisitor for NodeCounter {
    fn visit_rule_set(&mut self, _node: &oak_scss::ast::ScssRuleSet) {
        self.rule_sets += 1;
    }

    fn visit_mixin_declaration(&mut self, _node: &oak_scss::ast::ScssMixinDeclaration) {
        self.mixins += 1;
    }

    fn visit_function_declaration(&mut self, _node: &oak_scss::ast::ScssFunctionDeclaration) {
        self.functions += 1;
    }

    fn visit_variable_declaration(&mut self, _node: &oak_scss::ast::ScssVariableDeclaration) {
        self.variables += 1;
    }
}

#[test]
fn test_ast_build() {
    let scss_code = r#"
        $primary-color: #333;
        
        @mixin button {
            padding: 10px;
            border: 1px solid #ccc;
        }
        
        @function darken($color, $amount) {
            @return $color - $amount;
        }
        
        body {
            background-color: $primary-color;
            
            .container {
                padding: 20px;
            }
        }
    "#;

    let language = ScssLanguage::default();
    let builder = ScssBuilder::new(&language);
    let source = StringSource::new(scss_code);
    let mut cache = oak_core::builder::NoBuilderCache;

    let result = builder.build(&source, &[], &mut cache);
    assert!(result.result.is_ok());

    let ast = result.result.unwrap();

    // Test traversal
    let mut counter = NodeCounter::new();
    ast.traverse(&mut counter);

    assert_eq!(counter.rule_sets, 2); // body and .container
    assert_eq!(counter.mixins, 1);
    assert_eq!(counter.functions, 1);
    assert_eq!(counter.variables, 1);

    println!("AST build test passed!");
    println!("Rule sets: {}", counter.rule_sets);
    println!("Mixins: {}", counter.mixins);
    println!("Functions: {}", counter.functions);
    println!("Variables: {}", counter.variables);
}
