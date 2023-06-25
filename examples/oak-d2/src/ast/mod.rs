use core::range::Range;

/// The root of a D2 Abstract Syntax Tree.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct D2Root {
    /// The elements in the D2 diagram.
    pub elements: Vec<D2Element>,
    /// The span of the entire diagram in the source file.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A top-level element in a D2 diagram.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum D2Element {
    /// A shape definition.
    Shape(Shape),
    /// A connection between shapes.
    Connection(Connection),
}

/// A shape in a D2 diagram.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Shape {
    /// The identifier of the shape.
    pub id: String,
    /// The optional label of the shape.
    pub label: Option<String>,
}

/// A connection between shapes in a D2 diagram.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Connection {
    /// The source shape ID.
    pub from: String,
    /// The target shape ID.
    pub to: String,
    /// The span of the connection in the source file.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl std::fmt::Display for D2Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in &self.elements {
            match element {
                D2Element::Shape(s) => {
                    if let Some(label) = &s.label {
                        writeln!(f, "{}: {}", s.id, label)?;
                    }
                    else {
                        writeln!(f, "{}", s.id)?;
                    }
                }
                D2Element::Connection(c) => {
                    writeln!(f, "{} -> {}", c.from, c.to)?;
                }
            }
        }
        Ok(())
    }
}
