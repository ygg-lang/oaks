use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde_json;

/// Format annotation
/// 
/// This struct represents a formatting annotation extracted from the code.
#[derive(Debug, Clone)]
pub struct FormatAnnotation {
    /// The name of the annotation
    pub name: String,
    /// The parameters of the annotation
    pub params: Vec<AnnotationParam>,
    /// The span of the annotation in the source code
    pub span: core::ops::Range<usize>,
}

/// Annotation parameter
/// 
/// This struct represents a parameter of a formatting annotation.
#[derive(Debug, Clone)]
pub struct AnnotationParam {
    /// The name of the parameter
    pub name: String,
    /// The value of the parameter
    pub value: AnnotationValue,
}

/// Annotation value
/// 
/// This enum represents the value of an annotation parameter.
#[derive(Debug, Clone)]
pub enum AnnotationValue {
    /// A boolean value
    Bool(bool),
    /// An integer value
    Int(i64),
    /// A string value
    String(String),
    /// A float value
    Float(f64),
    /// A list of values
    List(Vec<AnnotationValue>),
    /// A map of key-value pairs
    Map(Vec<(String, AnnotationValue)>),
}

/// Annotation parser
/// 
/// This trait defines the interface for parsing annotations from different languages.
pub trait AnnotationParser {
    /// Parses annotations from the source code
    /// 
    /// # Parameters
    /// - `source`: The source code string
    /// 
    /// # Returns
    /// A vector of `FormatAnnotation` objects
    fn parse(&self, source: &str) -> Vec<FormatAnnotation>;
}

/// Annotation processor
/// 
/// This struct processes annotations and applies them to the formatting state.
pub struct AnnotationProcessor<P: AnnotationParser> {
    /// The annotation parser to use
    parser: P,
}

impl<P: AnnotationParser> AnnotationProcessor<P> {
    /// Creates a new annotation processor
    /// 
    /// # Parameters
    /// - `parser`: The annotation parser to use
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    /// Processes annotations from the source code
    /// 
    /// # Parameters
    /// - `source`: The source code string
    /// 
    /// # Returns
    /// A vector of `FormatAnnotation` objects
    pub fn process(&self, source: &str) -> Vec<FormatAnnotation> {
        self.parser.parse(source)
    }
}

#[cfg(feature = "serde")]
impl AnnotationValue {
    /// Converts an AnnotationValue to a serde_json::Value
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            AnnotationValue::Bool(v) => serde_json::Value::Bool(*v),
            AnnotationValue::Int(v) => serde_json::Value::Number(serde_json::Number::from(*v)),
            AnnotationValue::Float(v) => {
                if v.is_finite() {
                    serde_json::Value::Number(serde_json::Number::from_f64(*v).unwrap_or(serde_json::Number::from(0)))
                } else {
                    serde_json::Value::Null
                }
            }
            AnnotationValue::String(v) => serde_json::Value::String(v.clone()),
            AnnotationValue::List(values) => {
                serde_json::Value::Array(values.iter().map(|v| v.to_json()).collect())
            }
            AnnotationValue::Map(pairs) => {
                let json_map: serde_json::Map<String, serde_json::Value> = pairs.iter()
                    .map(|(k, v)| (k.clone(), v.to_json()))
                    .collect();
                serde_json::Value::Object(json_map)
            }
        }
    }
}
