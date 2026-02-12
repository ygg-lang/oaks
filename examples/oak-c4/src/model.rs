//! Model definitions for the C4 model for software architecture.
//!
//! This module provides structures for representing C4 diagrams, including
//! persons, software systems, containers, components, and their relationships.

/// Represents an element in a C4 diagram.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum C4Element {
    /// A person using the software system.
    Person(Person),
    /// The highest level of abstraction (the software system being modeled).
    SoftwareSystem(SoftwareSystem),
    /// A container (e.g., a web application, database, etc.).
    Container(Container),
    /// A component within a container.
    Component(Component),
    /// A relationship between C4 elements.
    Relationship(Relationship),
}

/// Represents a person in a C4 diagram.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Person {
    /// The name of the person.
    pub name: String,
    /// An optional description of the person's role or responsibilities.
    pub description: Option<String>,
}

impl Person {
    /// Creates a new person with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), description: None }
    }

    /// Sets the description of the person.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Represents a software system in a C4 diagram.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoftwareSystem {
    /// The name of the software system.
    pub name: String,
    /// An optional description of the software system's purpose.
    pub description: Option<String>,
}

impl SoftwareSystem {
    /// Creates a new software system with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), description: None }
    }

    /// Sets the description of the software system.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Represents a container in a C4 diagram.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Container {
    /// The name of the container.
    pub name: String,
    /// The technology used by the container (e.g., "Java", "PostgreSQL").
    pub technology: Option<String>,
    /// An optional description of the container's role.
    pub description: Option<String>,
}

impl Container {
    /// Creates a new container with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), technology: None, description: None }
    }

    /// Sets the technology used by the container.
    pub fn with_technology(mut self, technology: impl Into<String>) -> Self {
        self.technology = Some(technology.into());
        self
    }

    /// Sets the description of the container.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Represents a component in a C4 diagram.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Component {
    /// The name of the component.
    pub name: String,
    /// The technology used by the component.
    pub technology: Option<String>,
    /// An optional description of the component's role.
    pub description: Option<String>,
}

impl Component {
    /// Creates a new component with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), technology: None, description: None }
    }

    /// Sets the technology used by the component.
    pub fn with_technology(mut self, technology: impl Into<String>) -> Self {
        self.technology = Some(technology.into());
        self
    }

    /// Sets the description of the component.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Represents a relationship between two C4 elements.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Relationship {
    /// The source element name.
    pub source: String,
    /// The target element name.
    pub target: String,
    /// A label describing the relationship.
    pub label: String,
    /// The technology used for the relationship (e.g., "HTTPS", "JDBC").
    pub technology: Option<String>,
}

impl Relationship {
    /// Creates a new relationship between source and target with a label.
    pub fn new(source: impl Into<String>, target: impl Into<String>, label: impl Into<String>) -> Self {
        Self { source: source.into(), target: target.into(), label: label.into(), technology: None }
    }

    /// Sets the technology used for the relationship.
    pub fn with_technology(mut self, technology: impl Into<String>) -> Self {
        self.technology = Some(technology.into());
        self
    }
}

/// A collection of C4 elements forming a diagram.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct C4Diagram {
    /// The elements in the diagram.
    pub elements: Vec<C4Element>,
}

impl C4Diagram {
    /// Creates a new empty C4 diagram.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an element to the diagram.
    pub fn with_element(mut self, element: impl Into<C4Element>) -> Self {
        self.elements.push(element.into());
        self
    }

    /// Adds a person to the diagram.
    pub fn person(self, name: impl Into<String>, description: impl Into<String>) -> Self {
        self.with_element(Person::new(name).with_description(description))
    }

    /// Adds a software system to the diagram.
    pub fn software_system(self, name: impl Into<String>, description: impl Into<String>) -> Self {
        self.with_element(SoftwareSystem::new(name).with_description(description))
    }

    /// Adds a container to the diagram.
    pub fn container(self, name: impl Into<String>, technology: impl Into<String>, description: impl Into<String>) -> Self {
        self.with_element(Container::new(name).with_technology(technology).with_description(description))
    }

    /// Adds a component to the diagram.
    pub fn component(self, name: impl Into<String>, technology: impl Into<String>, description: impl Into<String>) -> Self {
        self.with_element(Component::new(name).with_technology(technology).with_description(description))
    }

    /// Adds a relationship to the diagram.
    pub fn rel(self, source: impl Into<String>, target: impl Into<String>, label: impl Into<String>) -> Self {
        self.with_element(Relationship::new(source, target, label))
    }

    /// Adds a relationship with technology to the diagram.
    pub fn rel_with_tech(self, source: impl Into<String>, target: impl Into<String>, label: impl Into<String>, technology: impl Into<String>) -> Self {
        self.with_element(Relationship::new(source, target, label).with_technology(technology))
    }
}

impl From<Person> for C4Element {
    fn from(p: Person) -> Self {
        C4Element::Person(p)
    }
}

impl From<SoftwareSystem> for C4Element {
    fn from(s: SoftwareSystem) -> Self {
        C4Element::SoftwareSystem(s)
    }
}

impl From<Container> for C4Element {
    fn from(c: Container) -> Self {
        C4Element::Container(c)
    }
}

impl From<Component> for C4Element {
    fn from(c: Component) -> Self {
        C4Element::Component(c)
    }
}

impl From<Relationship> for C4Element {
    fn from(r: Relationship) -> Self {
        C4Element::Relationship(r)
    }
}
