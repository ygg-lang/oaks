use oak_vbnet::{VbNetLanguage, parse};

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
    let result = parse(code, &[], &mut (), &language);

    match result.result {
        Ok(_) => println!("Parsing successful!"),
        Err(error) => println!("Parsing failed: {:?}", error),
    }
}
