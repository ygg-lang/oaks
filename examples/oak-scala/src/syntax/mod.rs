pub use crate::kind::ScalaSyntaxKind;

impl oak_core::SyntaxKind for ScalaSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(
            self,
            ScalaSyntaxKind::Whitespace
                | ScalaSyntaxKind::Newline
                | ScalaSyntaxKind::LineComment
                | ScalaSyntaxKind::BlockComment
                | ScalaSyntaxKind::DocComment
        )
    }

    fn is_comment(&self) -> bool {
        matches!(self, ScalaSyntaxKind::LineComment | ScalaSyntaxKind::BlockComment | ScalaSyntaxKind::DocComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, ScalaSyntaxKind::Whitespace | ScalaSyntaxKind::Newline)
    }

    fn is_token_type(&self) -> bool {
        !self.is_element_type()
    }

    fn is_element_type(&self) -> bool {
        matches!(self, ScalaSyntaxKind::SourceFile | ScalaSyntaxKind::ErrorNode)
    }
}
