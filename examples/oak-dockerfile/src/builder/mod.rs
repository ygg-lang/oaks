use crate::{DockerfileParser, ast::*, language::DockerfileLanguage, lexer::token_type::DockerfileTokenType, parser::element_type::DockerfileElementType};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// AST builder for the Dockerfile language.
#[derive(Clone, Copy)]
pub struct DockerfileBuilder<'config> {
    /// Language configuration.
    config: &'config DockerfileLanguage,
}

impl<'config> DockerfileBuilder<'config> {
    /// Creates a new `DockerfileBuilder` with the given language configuration.
    pub fn new(config: &'config DockerfileLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<DockerfileLanguage> for DockerfileBuilder<'config> {
    /// Builds the Dockerfile AST from the green tree.
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<DockerfileLanguage>) -> BuildOutput<DockerfileLanguage> {
        let parser = DockerfileParser::new(self.config);

        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> DockerfileBuilder<'config> {
    /// Builds the AST root from the green tree.
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, DockerfileLanguage>, source: &SourceText) -> Result<DockerfileRoot, OakError> {
        let root_node = RedNode::new(green_tree, 0);
        let mut instructions = Vec::new();

        for child in root_node.children() {
            if let RedTree::Node(n) = child {
                match n.green.kind {
                    DockerfileElementType::From => instructions.push(self.build_from(n, source)?),
                    DockerfileElementType::Run => instructions.push(self.build_run(n, source)?),
                    DockerfileElementType::Copy => instructions.push(self.build_copy(n, source)?),
                    DockerfileElementType::Add => instructions.push(self.build_add(n, source)?),
                    DockerfileElementType::Workdir => instructions.push(self.build_workdir(n, source)?),
                    DockerfileElementType::Expose => instructions.push(self.build_expose(n, source)?),
                    DockerfileElementType::Env => instructions.push(self.build_env(n, source)?),
                    DockerfileElementType::Cmd => instructions.push(self.build_cmd(n, source)?),
                    DockerfileElementType::Entrypoint => instructions.push(self.build_entrypoint(n, source)?),
                    DockerfileElementType::Volume => instructions.push(self.build_volume(n, source)?),
                    DockerfileElementType::User => instructions.push(self.build_user(n, source)?),
                    DockerfileElementType::Label => instructions.push(self.build_label(n, source)?),
                    DockerfileElementType::Arg => instructions.push(self.build_arg(n, source)?),
                    DockerfileElementType::Stopsignal => instructions.push(self.build_stopsignal(n, source)?),
                    DockerfileElementType::Healthcheck => instructions.push(self.build_healthcheck(n, source)?),
                    DockerfileElementType::Shell => instructions.push(self.build_shell(n, source)?),
                    DockerfileElementType::Onbuild => instructions.push(self.build_onbuild(n, source)?),
                    _ => {}
                }
            }
        }

        Ok(DockerfileRoot { instructions })
    }

    fn build_from(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut image = String::new();
        let mut children = node.children();

        // Skip keyword
        children.next();

        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    image.push_str(source.get_text_in(t.span()).as_ref());
                }
            }
        }

        Ok(Instruction::From { image: image.trim().to_string(), tag: None, span: node.span() })
    }

    fn build_run(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut command = String::new();
        let mut children = node.children();

        // Skip keyword
        children.next();

        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    command.push_str(source.get_text_in(t.span()).as_ref());
                }
            }
        }

        Ok(Instruction::Run { command: command.trim().to_string(), span: node.span() })
    }

    fn build_copy(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut args = Vec::new();
        let mut children = node.children();

        // Skip keyword
        children.next();

        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    args.push(source.get_text_in(t.span()).to_string());
                }
            }
        }

        Ok(Instruction::Copy { src: args.get(0).cloned().unwrap_or_default(), dest: args.get(1).cloned().unwrap_or_default(), span: node.span() })
    }

    fn build_add(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut args = Vec::new();
        let mut children = node.children();

        // Skip keyword
        children.next();

        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    args.push(source.get_text_in(t.span()).to_string());
                }
            }
        }

        Ok(Instruction::Add { src: args.get(0).cloned().unwrap_or_default(), dest: args.get(1).cloned().unwrap_or_default(), span: node.span() })
    }

    fn build_workdir(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut path = String::new();
        let mut children = node.children();
        children.next(); // skip keyword
        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    path.push_str(source.get_text_in(t.span()).as_ref());
                }
            }
        }
        Ok(Instruction::Workdir { path: path.trim().to_string(), span: node.span() })
    }

    fn build_expose(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut port = String::new();
        let mut children = node.children();
        children.next(); // skip keyword
        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    port.push_str(source.get_text_in(t.span()).as_ref());
                }
            }
        }
        Ok(Instruction::Expose { port: port.trim().to_string(), span: node.span() })
    }

    fn build_env(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut key = String::new();
        let mut value = String::new();
        let mut children = node.children();
        children.next(); // skip keyword

        let mut parts = Vec::new();
        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    parts.push(source.get_text_in(t.span()).to_string());
                }
            }
        }

        if parts.len() >= 2 {
            key = parts[0].clone();
            value = parts[1..].join(" ");
        }
        else if !parts.is_empty() {
            key = parts[0].clone();
        }

        Ok(Instruction::Env { key, value, span: node.span() })
    }

    fn build_cmd(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut command = String::new();
        let mut children = node.children();
        children.next(); // skip keyword
        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    command.push_str(source.get_text_in(t.span()).as_ref());
                }
            }
        }
        Ok(Instruction::Cmd { command: command.trim().to_string(), span: node.span() })
    }

    fn build_entrypoint(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut command = String::new();
        let mut children = node.children();
        children.next(); // skip keyword
        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    command.push_str(source.get_text_in(t.span()).as_ref());
                }
            }
        }
        Ok(Instruction::Entrypoint { command: command.trim().to_string(), span: node.span() })
    }

    fn build_volume(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut path = String::new();
        let mut children = node.children();
        children.next(); // skip keyword
        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    path.push_str(source.get_text_in(t.span()).as_ref());
                }
            }
        }
        Ok(Instruction::Volume { path: path.trim().to_string(), span: node.span() })
    }

    fn build_user(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut user = String::new();
        let mut children = node.children();
        children.next(); // skip keyword
        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    user.push_str(source.get_text_in(t.span()).as_ref());
                }
            }
        }
        Ok(Instruction::User { user: user.trim().to_string(), span: node.span() })
    }

    fn build_label(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut key = String::new();
        let mut value = String::new();
        let mut children = node.children();
        children.next(); // skip keyword

        let mut parts = Vec::new();
        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    parts.push(source.get_text_in(t.span()).to_string());
                }
            }
        }

        if parts.len() >= 2 {
            key = parts[0].clone();
            value = parts[1..].join(" ");
        }
        else if !parts.is_empty() {
            key = parts[0].clone();
        }

        Ok(Instruction::Label { key, value, span: node.span() })
    }

    fn build_arg(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut name = String::new();
        let mut default = None;
        let mut children = node.children();
        children.next(); // skip keyword

        let mut parts = Vec::new();
        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    parts.push(source.get_text_in(t.span()).to_string());
                }
            }
        }

        if !parts.is_empty() {
            name = parts[0].clone();
            if parts.len() > 1 {
                default = Some(parts[1..].join(" "));
            }
        }

        Ok(Instruction::Arg { name, default, span: node.span() })
    }

    fn build_stopsignal(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut signal = String::new();
        let mut children = node.children();
        children.next(); // skip keyword
        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    signal.push_str(source.get_text_in(t.span()).as_ref());
                }
            }
        }
        Ok(Instruction::Stopsignal { signal: signal.trim().to_string(), span: node.span() })
    }

    fn build_healthcheck(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut command = String::new();
        let mut children = node.children();
        children.next(); // skip keyword
        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    command.push_str(source.get_text_in(t.span()).as_ref());
                }
            }
        }
        Ok(Instruction::Healthcheck { command: command.trim().to_string(), span: node.span() })
    }

    fn build_shell(&self, node: RedNode<DockerfileLanguage>, source: &SourceText) -> Result<Instruction, OakError> {
        let mut shell = String::new();
        let mut children = node.children();
        children.next(); // skip keyword
        for child in children {
            if let RedTree::Leaf(t) = child {
                if t.kind != DockerfileTokenType::Whitespace {
                    shell.push_str(source.get_text_in(t.span()).as_ref());
                }
            }
        }
        Ok(Instruction::Shell { shell: shell.trim().to_string(), span: node.span() })
    }

    fn build_onbuild(&self, node: RedNode<DockerfileLanguage>, _source: &SourceText) -> Result<Instruction, OakError> {
        // Simple implementation for now
        Ok(Instruction::Onbuild { instruction: Box::new(Instruction::Run { command: "ONBUILD placeholder".to_string(), span: node.span() }), span: node.span() })
    }
}

fn text(source: &SourceText, span: core::range::Range<usize>) -> String {
    source.get_text_in(span).to_string()
}
