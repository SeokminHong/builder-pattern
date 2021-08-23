use super::attributes::FieldAttributes;

use proc_macro2::Ident;
use std::cmp::Ordering;
use syn::{Attribute, Type, Visibility};

pub struct Field {
    pub vis: Visibility,
    pub ident: Ident,
    pub ty: Type,
    pub attrs: FieldAttributes,
}

impl Field {
    pub fn documents(&self) -> Vec<Attribute> {
        self.attrs
            .documents
            .iter()
            .filter(|a| a.path.is_ident("doc"))
            .map(|a| a.to_owned())
            .collect()
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Field) -> Ordering {
        self.ident.cmp(&other.ident)
    }
}

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Field) -> Option<Ordering> {
        Some(self.ident.cmp(&other.ident))
    }
}

impl Eq for Field {}

impl PartialEq for Field {
    fn eq(&self, other: &Field) -> bool {
        self.ident.eq(&other.ident)
    }
}
