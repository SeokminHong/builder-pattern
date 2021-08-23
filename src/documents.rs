use crate::attributes::FieldAttributes;
use crate::structure::StructureInput;

use syn::Attribute;

pub trait DocumentsGenerator {
    fn generate_documents(&self) -> Vec<Attribute>;
}

impl DocumentsGenerator for StructureInput {
    fn generate_documents(&self) -> Vec<Attribute> {
        let mut docs: Vec<Attribute> = Vec::new();

        docs
    }
}

impl DocumentsGenerator for FieldAttributes {
    fn generate_documents(&self) -> Vec<Attribute> {
        let mut docs: Vec<Attribute> = Vec::new();

        docs
    }
}
