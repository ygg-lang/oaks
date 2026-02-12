use core::range::Range;
use oak_c4::model::C4Element;

/// The root of a Structurizr Abstract Syntax Tree.
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructurizrRoot {
    /// The name of the workspace.
    pub name: String,
    /// The description of the workspace.
    pub description: Option<String>,
    /// The C4 elements in the diagram.
    pub elements: Vec<C4Element>,
    /// The span of the entire diagram in the source file.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl StructurizrRoot {
    /// Creates a new empty Structurizr document.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), ..Self::default() }
    }

    /// Sets the description of the workspace.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a C4 element to the document.
    pub fn with_element(mut self, element: impl Into<C4Element>) -> Self {
        self.elements.push(element.into());
        self
    }

    /// Sets the span of the document.
    pub fn with_span(mut self, span: Range<usize>) -> Self {
        self.span = span;
        self
    }
}

impl From<oak_c4::model::C4Diagram> for StructurizrRoot {
    fn from(diagram: oak_c4::model::C4Diagram) -> Self {
        Self { elements: diagram.elements, ..Self::default() }
    }
}

impl std::fmt::Display for StructurizrRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "workspace \"{}\" \"{}\" {{", self.name, self.description.as_deref().unwrap_or(""))?;
        writeln!(f, "    model {{")?;
        for element in &self.elements {
            match element {
                C4Element::Person(p) => {
                    writeln!(f, "        {} = person \"{}\" \"{}\"", p.name, p.name, p.description.as_deref().unwrap_or(""))?;
                }
                C4Element::SoftwareSystem(s) => {
                    writeln!(f, "        {} = softwareSystem \"{}\" \"{}\"", s.name, s.name, s.description.as_deref().unwrap_or(""))?;
                }
                C4Element::Container(c) => {
                    writeln!(f, "        {} = container \"{}\" \"{}\" \"{}\"", c.name, c.name, c.description.as_deref().unwrap_or(""), c.technology.as_deref().unwrap_or(""))?;
                }
                C4Element::Component(c) => {
                    writeln!(f, "        {} = component \"{}\" \"{}\" \"{}\"", c.name, c.name, c.description.as_deref().unwrap_or(""), c.technology.as_deref().unwrap_or(""))?;
                }
                C4Element::Relationship(r) => {
                    writeln!(f, "        {} -> {} \"{}\" \"{}\"", r.source, r.target, r.label, r.technology.as_deref().unwrap_or(""))?;
                }
            }
        }
        writeln!(f, "    }}")?;
        writeln!(f, "}}")?;
        Ok(())
    }
}
