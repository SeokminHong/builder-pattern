use crate::attributes::FieldAttributes;
use crate::struct_input::StructInput;

use syn::Attribute;

pub trait DocumentsGenerator {
    fn generate_documents(&self) -> Vec<Attribute>;
}

impl DocumentsGenerator for StructInput {
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
