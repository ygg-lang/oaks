use crate::{kind::PowerShellSyntaxKind, language::PowerShellLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, PowerShellLanguage>;

pub struct PowerShellLexer<'config> {
    config: &'config PowerShellLanguage,
}

impl<'config> PowerShellLexer<'config> {
    pub fn new(config: &'config PowerShellLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(PowerShellSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(PowerShellSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(PowerShellSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);
            
            // 单行注释
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            
            state.add_token(PowerShellSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else if state.peek() == Some('<') && state.peek_next_n(1) == Some('#') {
            // 多行注释 <# ... #>
            state.advance(2);
            
            while let Some(ch) = state.peek() {
                if ch == '#' && state.peek_next_n(1) == Some('>') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            
            state.add_token(PowerShellSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);
                
                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        break;
                    }
                    else if ch == '`' && quote == '"' {
                        // PowerShell 转义字符
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                
                state.add_token(PowerShellSyntaxKind::String, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理 here-string
    fn lex_here_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('@') {
            if state.peek_next_n(1) == Some('"') {
                // @"....."@
                state.advance(2);
                
                while let Some(ch) = state.peek() {
                    if ch == '"' && state.peek_next_n(1) == Some('@') {
                        state.advance(2);
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                
                state.add_token(PowerShellSyntaxKind::String, start_pos, state.get_position());
                true
            }
            else if state.peek_next_n(1) == Some('\'') {
                // @'.....'@
                state.advance(2);
                
                while let Some(ch) = state.peek() {
                    if ch == '\'' && state.peek_next_n(1) == Some('@') {
                        state.advance(2);
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                
                state.add_token(PowerShellSyntaxKind::String, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
                
                // 整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
                
                // 小数部分
                if state.peek() == Some('.') && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                
                // 科学计数法
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
                
                state.add_token(PowerShellSyntaxKind::Number, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理变量
    fn lex_variable(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('$') = state.peek() {
            state.advance(1);
            
            // 特殊变量如 $_, $?, $^, $$
            if let Some(ch) = state.peek() {
                if ch == '_' || ch == '?' || ch == '^' || ch == '$' {
                    state.advance(1);
                    state.add_token(PowerShellSyntaxKind::Variable, start_pos, state.get_position());
                    return true;
                }
            }
            
            // 普通变量名
            if let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch == '_' {
                    state.advance(ch.len_utf8());
                    
                    while let Some(ch) = state.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    
                    state.add_token(PowerShellSyntaxKind::Variable, start_pos, state.get_position());
                    true
                }
                else {
                    // 只有 $ 符号
                    state.add_token(PowerShellSyntaxKind::Variable, start_pos, state.get_position());
                    true
                }
            }
            else {
                state.add_token(PowerShellSyntaxKind::Variable, start_pos, state.get_position());
                true
            }
        }
        else {
            false
        }
    }

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                
                let text = state.get_text_range(start_pos, state.get_position());
                let kind = match text.to_lowercase().as_str() {
                    "if" | "else" | "elseif" | "switch" | "for" | "foreach" | "while" | "do" |
                    "function" | "filter" | "workflow" | "class" | "enum" | "param" | "begin" |
                    "process" | "end" | "try" | "catch" | "finally" | "throw" | "return" |
                    "break" | "continue" | "exit" | "where" | "select" | "sort" | "group" |
                    "measure" | "compare" | "tee" | "out" | "export" | "import" | "new" |
                    "set" | "get" | "add" | "remove" | "clear" | "copy" | "move" | "rename" |
                    "test" | "invoke" | "start" | "stop" | "restart" | "suspend" | "resume" |
                    "wait" | "receive" | "send" | "read" | "write" | "format" | "convert" |
                    "join" | "split" | "replace" | "match" | "like" | "contains" | "in" |
                    "is" | "as" | "and" | "or" | "not" | "xor" | "band" | "bor" | "bnot" |
                    "bxor" | "shl" | "shr" | "eq" | "ne" | "gt" | "ge" | "lt" | "le" |
                    "ieq" | "ine" | "igt" | "ige" | "ilt" | "ile" | "ceq" | "cne" | "cgt" |
                    "cge" | "clt" | "cle" | "true" | "false" | "null" => PowerShellSyntaxKind::Keyword,
                    _ => PowerShellSyntaxKind::Identifier,
                };
                
                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理操作符
    fn lex_operator(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // 多字符操作符
        let multi_char_ops = [
            "++", "--", "+=", "-=", "*=", "/=", "%=", "==", "!=", "<=", ">=", 
            "&&", "||", "..", "-eq", "-ne", "-gt", "-ge", "-lt", "-le",
            "-like", "-notlike", "-match", "-notmatch", "-contains", "-notcontains",
            "-in", "-notin", "-replace", "-split", "-join", "-is", "-isnot",
            "-as", "-and", "-or", "-not", "-xor", "-band", "-bor", "-bnot",
            "-bxor", "-shl", "-shr", "-ieq", "-ine", "-igt", "-ige", "-ilt",
            "-ile", "-ceq", "-cne", "-cgt", "-cge", "-clt", "-cle"
        ];

        for op in &multi_char_ops {
            if state.peek_string(op.len()) == Some(op.to_string()) {
                state.advance(op.len());
                state.add_token(PowerShellSyntaxKind::Operator, start_pos, state.get_position());
                return true;
            }
        }

        // 单字符操作符
        if let Some(ch) = state.peek() {
            match ch {
                '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '!' | '&' | '|' | '^' | '~' => {
                    state.advance(1);
                    state.add_token(PowerShellSyntaxKind::Operator, start_pos, state.get_position());
                    true
                }
                _ => false
            }
        }
        else {
            false
        }
    }

    /// 处理分隔符
    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            match ch {
                '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';' | '.' | ':' => {
                    state.advance(1);
                    state.add_token(PowerShellSyntaxKind::Delimiter, start_pos, state.get_position());
                    true
                }
                _ => false
            }
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<PowerShellLanguage> for PowerShellLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<PowerShellSyntaxKind> {
        let mut state = State::new(source);
        
        while !state.is_at_end() {
            if self.skip_whitespace(&mut state) {
                continue;
            }
            
            if self.lex_newline(&mut state) {
                continue;
            }
            
            if self.lex_comment(&mut state) {
                continue;
            }
            
            if self.lex_here_string(&mut state) {
                continue;
            }
            
            if self.lex_string(&mut state) {
                continue;
            }
            
            if self.lex_number(&mut state) {
                continue;
            }
            
            if self.lex_variable(&mut state) {
                continue;
            }
            
            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }
            
            if self.lex_operator(&mut state) {
                continue;
            }
            
            if self.lex_delimiter(&mut state) {
                continue;
            }
            
            // 如果没有匹配任何规则，跳过当前字符
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
            }
        }
        
        // 添加 EOF kind
        let pos = state.get_position();
        state.add_token(PowerShellSyntaxKind::Eof, pos, pos);
        
        state.finish()
    }
}
