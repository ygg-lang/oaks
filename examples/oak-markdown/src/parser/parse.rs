use crate::{
    kind::MarkdownSyntaxKind,
    parser::{MarkdownParser, State},
};
use oak_core::{Arc, GreenNode, OakError, source::Source};

impl<'config> MarkdownParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<Arc<GreenNode<MarkdownSyntaxKind>>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            let item_checkpoint = state.checkpoint();
            if let Some(kind) = state.peek_kind() {
                match kind {
                    MarkdownSyntaxKind::Heading1 | MarkdownSyntaxKind::Heading2 | MarkdownSyntaxKind::Heading3 |
                    MarkdownSyntaxKind::Heading4 | MarkdownSyntaxKind::Heading5 | MarkdownSyntaxKind::Heading6 => {
                        // 消耗 # 标记
                        state.bump(); 
                        
                        // 消耗标题内容直到换行
                        while state.not_at_end() {
                            if let Some(next_kind) = state.peek_kind() {
                                if next_kind == MarkdownSyntaxKind::Newline {
                                    break;
                                }
                            }
                            state.bump();
                        }
                        
                        // 结束当前节点，构建标题节点
                        state.finish_at(item_checkpoint, kind.into());
                    }
                    MarkdownSyntaxKind::Newline => {
                        state.bump();
                    }
                    _ => {
                        // 默认处理：推进一个 token
                        state.advance();
                    }
                }
            } else {
                state.advance();
            }
        }

        state.finish_at(checkpoint, MarkdownSyntaxKind::Root);
        Ok(state.builder.last_node().expect("Failed to build Root node"))
    }
}
