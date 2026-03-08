#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! High-level formatting library for the Oak language framework.
//!
//! This crate provides high-level formatting logic, including configuration management,
//! annotation processing, and language-specific formatting rules.

pub extern crate alloc;
extern crate self as oak_formatter;

// Public modules
/// Annotation handling and processing
pub mod annotation;
/// Formatting configuration
pub mod config;
/// Error types for formatting
pub mod errors;
/// Language-specific formatters
pub mod formatters;

// Re-export commonly used types
pub use crate::{
    annotation::{AnnotationParam, AnnotationParser, AnnotationProcessor, AnnotationValue, FormatAnnotation},
    config::{CommonFormatterConfig, IndentStyle, LineEnding},
    errors::FormatResult,
};

pub use crate::formatters::{Formatter, GenericFormatter};
pub use oak_core::language::Language;
