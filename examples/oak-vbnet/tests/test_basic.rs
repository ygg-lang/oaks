use oak_vbnet::parse;

fn main() {
    let code = r#"
    Imports System
    
    Public Class TestClass
        Public Sub TestMethod()
            Console.WriteLine("Hello World")
        End Sub
    End Class
    "#;

    match parse(code) {
        Ok(root) => println!("Parsing successful: {:?}", root),
        Err(error) => println!("Parsing error: {:?}", error),
    }
}
