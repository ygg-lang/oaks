use oak_core::{
    parser::{ParseSession, parse_with_lexer},
    source::StringSource,
};
use oak_vbnet::{VbNetLanguage, VbNetParser};

fn main() {
    let code = r#"
    Public Class TestClass
        Public Sub TestFor()
            For i As Integer = 1 To 10 Step 2
                Console.WriteLine(i)
            Next
        End Sub
        
        Public Sub TestForEach(ByVal items As List(Of String))
            For Each item As String In items
                Console.WriteLine(item)
            Next
        End Sub
    End Class
    "#;

    let language = VbNetLanguage::new();
    let parser = VbNetParser::new(&language);
    let source = StringSource::new(code);
    let mut cache = ParseSession::new();

    let result = parse_with_lexer(&crate::lexer::VbNetLexer::new(&language), &source, &[], &mut cache, |state| {
        let cp = (0, 0);
        while state.not_at_end() {
            parser.parse_statement(state)?;
        }
        Ok(state.finish_at(cp, crate::parser::element_type::VbNetElementType::Root))
    });

    match result.result {
        Ok(_) => println!("Parsing successful!"),
        Err(error) => println!("Parsing failed: {:?}", error),
    }
}
