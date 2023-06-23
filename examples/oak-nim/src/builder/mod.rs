#![doc = include_str!("readme.md")]
use crate::NimLanguage;
use oak_core::Builder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NimRoot {
    pub items: Vec<String>,
}

pub struct NimBuilder<'config> {
    pub config: &'config NimLanguage,
}

impl<'config> NimBuilder<'config> {
    pub fn new(config: &'config NimLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<NimLanguage> for NimBuilder<'config> {
    fn build<'a, S: oak_core::source::Source + ?Sized>(&self, _source: &S, _edits: &[oak_core::TextEdit], _cache: &'a mut impl oak_core::BuilderCache<NimLanguage>) -> oak_core::errors::OakDiagnostics<NimRoot> {
        oak_core::errors::OakDiagnostics { result: Ok(NimRoot { items: vec![] }), diagnostics: vec![] }
    }
}
