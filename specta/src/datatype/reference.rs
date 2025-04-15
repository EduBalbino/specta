//! Helpers for generating [Type::reference] implementations.

use std::collections::BTreeMap;

use crate::SpectaID;

use super::{DataType, Generic};

/// A reference datatype.
///
/// TODO: Explain how to construct this.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Reference {
    pub(crate) sid: SpectaID,
    pub(crate) generics: BTreeMap<Generic, DataType>,
    pub(crate) inline: bool,
}

impl Reference {
    /// TODO: Explain invariant.
    pub fn construct(
        sid: SpectaID,
        generics: impl Into<BTreeMap<Generic, DataType>>,
        inline: bool,
    ) -> Self {
        Self {
            sid,
            generics: generics.into(),
            inline,
        }
    }

    pub fn sid(&self) -> SpectaID {
        self.sid
    }

    pub fn generics(&self) -> &BTreeMap<Generic, DataType> {
        &self.generics
    }

    pub fn inline(&self) -> bool {
        self.inline
    }
}

impl From<Reference> for DataType {
    fn from(r: Reference) -> Self {
        Self::Reference(r)
    }
}
