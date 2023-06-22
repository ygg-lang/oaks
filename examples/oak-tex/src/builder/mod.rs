use crate::{ast::*, kind::TexSyntaxKind, language::TexLanguage, parser::TexParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, TextEdit, source::Source};

/// TeX 语言的 AST 构建器
#[derive(Clone)]
pub struct TexBuilder<'config> {
    /// 语言配置
    config: &'config TexLanguage,
}

impl<'config> TexBuilder<'config> {
    /// 创建新的 TeX 构建器
    pub fn new(config: &'config TexLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<TexLanguage> for TexBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<TexLanguage>) -> oak_core::builder::BuildOutput<TexLanguage> {
        let parser = TexParser::new(self.config);

        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(green_tree) => match self.build_root_internal(green_tree, source) {
                Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                Err(build_error) => {
                    let mut diagnostics = parse_result.diagnostics;
                    diagnostics.push(build_error.clone());
                    OakDiagnostics { result: Err(build_error), diagnostics }
                }
            },
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> TexBuilder<'config> {
    /// 构建根节点
    fn build_root_internal<S: Source + ?Sized>(&self, green_tree: &GreenNode<TexLanguage>, source: &S) -> Result<TexRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        self.build_content(red_root, source)
    }

    fn build_content<S: Source + ?Sized>(&self, node: RedNode<TexLanguage>, source: &S) -> Result<TexRoot, OakError> {
        let mut items = Vec::new();
        let children: Vec<_> = node.children().collect();

        if children.is_empty() {
            return Ok(TexRoot { span: node.span().into(), items });
        }

        let first_kind = children.first().unwrap().kind::<TexSyntaxKind>();
        let last_kind = children.last().unwrap().kind::<TexSyntaxKind>();

        let start = if is_delimiter(first_kind) { 1 } else { 0 };
        let end = if children.len() > start && is_delimiter(last_kind) { children.len() - 1 } else { children.len() };

        for i in start..end {
            if let Some(item) = self.build_item(children[i], source)? {
                items.push(item);
            }
        }

        Ok(TexRoot { span: node.span().into(), items })
    }

    fn build_item<S: Source + ?Sized>(&self, tree: RedTree<TexLanguage>, source: &S) -> Result<Option<TexItem>, OakError> {
        let kind: TexSyntaxKind = tree.kind();
        match kind {
            TexSyntaxKind::Command
            | TexSyntaxKind::BeginKeyword
            | TexSyntaxKind::EndKeyword
            | TexSyntaxKind::Frac
            | TexSyntaxKind::Sqrt
            | TexSyntaxKind::Sum
            | TexSyntaxKind::Int
            | TexSyntaxKind::Lim
            | TexSyntaxKind::SectionKeyword
            | TexSyntaxKind::SubsectionKeyword
            | TexSyntaxKind::SubsubsectionKeyword
            | TexSyntaxKind::ChapterKeyword
            | TexSyntaxKind::PartKeyword
            | TexSyntaxKind::TitleKeyword
            | TexSyntaxKind::AuthorKeyword
            | TexSyntaxKind::DateKeyword
            | TexSyntaxKind::MaketitleKeyword
            | TexSyntaxKind::TableofcontentsKeyword
            | TexSyntaxKind::ItemKeyword
            | TexSyntaxKind::LabelKeyword
            | TexSyntaxKind::RefKeyword
            | TexSyntaxKind::CiteKeyword
            | TexSyntaxKind::IncludegraphicsKeyword
            | TexSyntaxKind::TextbfKeyword
            | TexSyntaxKind::TextitKeyword
            | TexSyntaxKind::EmphKeyword
            | TexSyntaxKind::Alpha
            | TexSyntaxKind::Beta
            | TexSyntaxKind::Gamma
            | TexSyntaxKind::Delta
            | TexSyntaxKind::Epsilon
            | TexSyntaxKind::Zeta
            | TexSyntaxKind::Eta
            | TexSyntaxKind::Theta
            | TexSyntaxKind::Iota
            | TexSyntaxKind::Kappa
            | TexSyntaxKind::Lambda
            | TexSyntaxKind::Mu
            | TexSyntaxKind::Nu
            | TexSyntaxKind::Xi
            | TexSyntaxKind::Omicron
            | TexSyntaxKind::Pi
            | TexSyntaxKind::Rho
            | TexSyntaxKind::Sigma
            | TexSyntaxKind::Tau
            | TexSyntaxKind::Upsilon
            | TexSyntaxKind::Phi
            | TexSyntaxKind::Chi
            | TexSyntaxKind::Psi
            | TexSyntaxKind::Omega
            | TexSyntaxKind::VarEpsilon
            | TexSyntaxKind::VarTheta
            | TexSyntaxKind::VarKappa
            | TexSyntaxKind::VarPi
            | TexSyntaxKind::VarRho
            | TexSyntaxKind::VarSigma
            | TexSyntaxKind::VarPhi
            | TexSyntaxKind::UpperGamma
            | TexSyntaxKind::UpperDelta
            | TexSyntaxKind::UpperTheta
            | TexSyntaxKind::UpperLambda
            | TexSyntaxKind::UpperXi
            | TexSyntaxKind::UpperPi
            | TexSyntaxKind::UpperSigma
            | TexSyntaxKind::UpperUpsilon
            | TexSyntaxKind::UpperPhi
            | TexSyntaxKind::UpperPsi
            | TexSyntaxKind::UpperOmega => {
                if let Some(node) = tree.as_node() {
                    Ok(Some(TexItem::Command(self.build_command(node, source)?)))
                }
                else {
                    let name = match kind {
                        TexSyntaxKind::Sum => "sum".to_string(),
                        TexSyntaxKind::Int => "int".to_string(),
                        TexSyntaxKind::Lim => "lim".to_string(),
                        TexSyntaxKind::Frac => "frac".to_string(),
                        TexSyntaxKind::Sqrt => "sqrt".to_string(),
                        TexSyntaxKind::Alpha => "alpha".to_string(),
                        TexSyntaxKind::Beta => "beta".to_string(),
                        TexSyntaxKind::Gamma => "gamma".to_string(),
                        TexSyntaxKind::Delta => "delta".to_string(),
                        TexSyntaxKind::Epsilon => "epsilon".to_string(),
                        TexSyntaxKind::Zeta => "zeta".to_string(),
                        TexSyntaxKind::Eta => "eta".to_string(),
                        TexSyntaxKind::Theta => "theta".to_string(),
                        TexSyntaxKind::Iota => "iota".to_string(),
                        TexSyntaxKind::Kappa => "kappa".to_string(),
                        TexSyntaxKind::Lambda => "lambda".to_string(),
                        TexSyntaxKind::Mu => "mu".to_string(),
                        TexSyntaxKind::Nu => "nu".to_string(),
                        TexSyntaxKind::Xi => "xi".to_string(),
                        TexSyntaxKind::Omicron => "omicron".to_string(),
                        TexSyntaxKind::Pi => "pi".to_string(),
                        TexSyntaxKind::Rho => "rho".to_string(),
                        TexSyntaxKind::Sigma => "sigma".to_string(),
                        TexSyntaxKind::Tau => "tau".to_string(),
                        TexSyntaxKind::Upsilon => "upsilon".to_string(),
                        TexSyntaxKind::Phi => "phi".to_string(),
                        TexSyntaxKind::Chi => "chi".to_string(),
                        TexSyntaxKind::Psi => "psi".to_string(),
                        TexSyntaxKind::Omega => "omega".to_string(),
                        TexSyntaxKind::VarEpsilon => "varepsilon".to_string(),
                        TexSyntaxKind::VarTheta => "vartheta".to_string(),
                        TexSyntaxKind::VarKappa => "varkappa".to_string(),
                        TexSyntaxKind::VarPi => "varpi".to_string(),
                        TexSyntaxKind::VarRho => "varrho".to_string(),
                        TexSyntaxKind::VarSigma => "varsigma".to_string(),
                        TexSyntaxKind::VarPhi => "varphi".to_string(),
                        TexSyntaxKind::UpperGamma => "Gamma".to_string(),
                        TexSyntaxKind::UpperDelta => "Delta".to_string(),
                        TexSyntaxKind::UpperTheta => "Theta".to_string(),
                        TexSyntaxKind::UpperLambda => "Lambda".to_string(),
                        TexSyntaxKind::UpperXi => "Xi".to_string(),
                        TexSyntaxKind::UpperPi => "Pi".to_string(),
                        TexSyntaxKind::UpperSigma => "Sigma".to_string(),
                        TexSyntaxKind::UpperUpsilon => "Upsilon".to_string(),
                        TexSyntaxKind::UpperPhi => "Phi".to_string(),
                        TexSyntaxKind::UpperPsi => "Psi".to_string(),
                        TexSyntaxKind::UpperOmega => "Omega".to_string(),
                        _ => tree.text(source).trim_start_matches('\\').to_string(),
                    };
                    Ok(Some(TexItem::Command(TexCommand { span: tree.span().into(), name, arguments: Vec::new() })))
                }
            }
            TexSyntaxKind::Environment => Ok(Some(TexItem::Environment(self.build_environment(tree.as_node().unwrap(), source)?))),
            TexSyntaxKind::Group => Ok(Some(TexItem::Group(self.build_group(tree.as_node().unwrap(), source)?))),
            TexSyntaxKind::InlineMath | TexSyntaxKind::DisplayMath => Ok(Some(TexItem::Math(self.build_math(tree.as_node().unwrap(), source)?))),
            TexSyntaxKind::Superscript => Ok(Some(TexItem::Superscript(self.build_superscript(tree.as_node().unwrap(), source)?))),
            TexSyntaxKind::Subscript => Ok(Some(TexItem::Subscript(self.build_subscript(tree.as_node().unwrap(), source)?))),
            TexSyntaxKind::Identifier | TexSyntaxKind::Number | TexSyntaxKind::Text => Ok(Some(TexItem::Text { span: tree.span().into(), content: tree.text(source).to_string() })),
            TexSyntaxKind::Comment => Ok(Some(TexItem::Comment { span: tree.span().into(), content: tree.text(source).to_string() })),
            _ => {
                if tree.as_leaf().is_some() {
                    Ok(Some(TexItem::Text { span: tree.span().into(), content: tree.text(source).to_string() }))
                }
                else {
                    Ok(None)
                }
            }
        }
    }

    fn build_superscript<S: Source + ?Sized>(&self, node: RedNode<TexLanguage>, source: &S) -> Result<TexSuperscript, OakError> {
        let mut content = TexRoot::new(node.span().into());
        for child in node.children() {
            if child.kind::<TexSyntaxKind>() == TexSyntaxKind::Caret {
                continue;
            }
            if let Some(item) = self.build_item(child, source)? {
                content.items.push(item);
            }
        }
        Ok(TexSuperscript {
            span: node.span().into(),
            target: None, // Will be filled later by a post-processor if needed
            content: Box::new(content),
        })
    }

    fn build_subscript<S: Source + ?Sized>(&self, node: RedNode<TexLanguage>, source: &S) -> Result<TexSubscript, OakError> {
        let mut content = TexRoot::new(node.span().into());
        for child in node.children() {
            if child.kind::<TexSyntaxKind>() == TexSyntaxKind::Underscore {
                continue;
            }
            if let Some(item) = self.build_item(child, source)? {
                content.items.push(item);
            }
        }
        Ok(TexSubscript {
            span: node.span().into(),
            target: None, // Will be filled later by a post-processor if needed
            content: Box::new(content),
        })
    }

    fn build_command<S: Source + ?Sized>(&self, node: RedNode<TexLanguage>, source: &S) -> Result<TexCommand, OakError> {
        let mut name = String::new();
        let mut arguments = Vec::new();

        for child in node.children() {
            let kind: TexSyntaxKind = child.kind();
            match kind {
                TexSyntaxKind::Backslash
                | TexSyntaxKind::Command
                | TexSyntaxKind::BeginKeyword
                | TexSyntaxKind::EndKeyword
                | TexSyntaxKind::Frac
                | TexSyntaxKind::Sqrt
                | TexSyntaxKind::Sum
                | TexSyntaxKind::Int
                | TexSyntaxKind::Lim
                | TexSyntaxKind::SectionKeyword
                | TexSyntaxKind::SubsectionKeyword
                | TexSyntaxKind::SubsubsectionKeyword
                | TexSyntaxKind::ChapterKeyword
                | TexSyntaxKind::PartKeyword
                | TexSyntaxKind::TitleKeyword
                | TexSyntaxKind::AuthorKeyword
                | TexSyntaxKind::DateKeyword
                | TexSyntaxKind::MaketitleKeyword
                | TexSyntaxKind::TableofcontentsKeyword
                | TexSyntaxKind::ItemKeyword
                | TexSyntaxKind::LabelKeyword
                | TexSyntaxKind::RefKeyword
                | TexSyntaxKind::CiteKeyword
                | TexSyntaxKind::IncludegraphicsKeyword
                | TexSyntaxKind::TextbfKeyword
                | TexSyntaxKind::TextitKeyword
                | TexSyntaxKind::EmphKeyword
                | TexSyntaxKind::Alpha
                | TexSyntaxKind::Beta
                | TexSyntaxKind::Gamma
                | TexSyntaxKind::Delta
                | TexSyntaxKind::Epsilon
                | TexSyntaxKind::Zeta
                | TexSyntaxKind::Eta
                | TexSyntaxKind::Theta
                | TexSyntaxKind::Iota
                | TexSyntaxKind::Kappa
                | TexSyntaxKind::Lambda
                | TexSyntaxKind::Mu
                | TexSyntaxKind::Nu
                | TexSyntaxKind::Xi
                | TexSyntaxKind::Omicron
                | TexSyntaxKind::Pi
                | TexSyntaxKind::Rho
                | TexSyntaxKind::Sigma
                | TexSyntaxKind::Tau
                | TexSyntaxKind::Upsilon
                | TexSyntaxKind::Phi
                | TexSyntaxKind::Chi
                | TexSyntaxKind::Psi
                | TexSyntaxKind::Omega
                | TexSyntaxKind::VarEpsilon
                | TexSyntaxKind::VarTheta
                | TexSyntaxKind::VarKappa
                | TexSyntaxKind::VarPi
                | TexSyntaxKind::VarRho
                | TexSyntaxKind::VarSigma
                | TexSyntaxKind::VarPhi
                | TexSyntaxKind::UpperGamma
                | TexSyntaxKind::UpperDelta
                | TexSyntaxKind::UpperTheta
                | TexSyntaxKind::UpperLambda
                | TexSyntaxKind::UpperXi
                | TexSyntaxKind::UpperPi
                | TexSyntaxKind::UpperSigma
                | TexSyntaxKind::UpperUpsilon
                | TexSyntaxKind::UpperPhi
                | TexSyntaxKind::UpperPsi
                | TexSyntaxKind::UpperOmega
                | TexSyntaxKind::TextBf
                | TexSyntaxKind::TextIt
                | TexSyntaxKind::TextSc
                | TexSyntaxKind::TextTt
                | TexSyntaxKind::Emph
                | TexSyntaxKind::Underline => {
                    let text = child.text(source);
                    if text.starts_with('\\') {
                        name = text[1..].to_string();
                    }
                    else {
                        name = text.to_string();
                    }
                }
                TexSyntaxKind::OptionalArgument => {
                    arguments.push(TexArgument::Optional(self.build_content(child.as_node().unwrap(), source)?));
                }
                TexSyntaxKind::MandatoryArgument => {
                    arguments.push(TexArgument::Required(self.build_content(child.as_node().unwrap(), source)?));
                }
                _ => {}
            }
        }

        Ok(TexCommand { span: node.span().into(), name, arguments })
    }

    fn build_group<S: Source + ?Sized>(&self, node: RedNode<TexLanguage>, source: &S) -> Result<TexGroup, OakError> {
        Ok(TexGroup { span: node.span().into(), content: self.build_content(node, source)? })
    }

    fn build_math<S: Source + ?Sized>(&self, node: RedNode<TexLanguage>, source: &S) -> Result<TexMath, OakError> {
        let kind: TexSyntaxKind = node.kind();
        Ok(TexMath { span: node.span().into(), content: self.build_content(node, source)?, is_display: kind == TexSyntaxKind::DoubleDollar })
    }

    fn build_environment<S: Source + ?Sized>(&self, node: RedNode<TexLanguage>, source: &S) -> Result<TexEnvironment, OakError> {
        let mut name = String::new();
        let mut arguments = Vec::new();
        let mut content = TexRoot::new(node.span().into());

        for child in node.children() {
            let kind: TexSyntaxKind = child.kind();
            match kind {
                TexSyntaxKind::BeginEnvironment => {
                    for sub_child in child.as_node().unwrap().children() {
                        match sub_child.kind::<TexSyntaxKind>() {
                            TexSyntaxKind::MandatoryArgument => {
                                // The first mandatory argument is the environment name
                                if name.is_empty() {
                                    let arg_root = self.build_content(sub_child.as_node().unwrap(), source)?;
                                    for item in arg_root.items {
                                        if let TexItem::Text { content, .. } = item {
                                            name.push_str(&content);
                                        }
                                    }
                                }
                                else {
                                    arguments.push(TexArgument::Required(self.build_content(sub_child.as_node().unwrap(), source)?));
                                }
                            }
                            TexSyntaxKind::OptionalArgument => {
                                arguments.push(TexArgument::Optional(self.build_content(sub_child.as_node().unwrap(), source)?));
                            }
                            _ => {}
                        }
                    }
                }
                TexSyntaxKind::EndEnvironment => {}
                _ => {
                    if let Some(item) = self.build_item(child, source)? {
                        content.items.push(item);
                    }
                }
            }
        }

        Ok(TexEnvironment { span: node.span().into(), name, arguments, content })
    }
}

fn is_delimiter(kind: TexSyntaxKind) -> bool {
    matches!(kind, TexSyntaxKind::LeftBrace | TexSyntaxKind::RightBrace | TexSyntaxKind::LeftBracket | TexSyntaxKind::RightBracket | TexSyntaxKind::Dollar | TexSyntaxKind::DoubleDollar)
}
