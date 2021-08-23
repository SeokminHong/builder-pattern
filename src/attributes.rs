use syn::{Attribute, Expr};

pub struct FieldAttributes {
    pub default: Option<Expr>,
    pub hidden: bool,
    pub use_into: bool,
    pub validator: Option<Expr>,
    pub documents: Vec<Attribute>,
}

impl Default for FieldAttributes {
    fn default() -> Self {
        FieldAttributes {
            default: None,
            hidden: false,
            use_into: false,
            validator: None,
            documents: vec![],
        }
    }
}

impl From<Vec<Attribute>> for FieldAttributes {
    fn from(attrs: Vec<Attribute>) -> FieldAttributes {
        let mut attributes = FieldAttributes::default();
        attrs.iter().for_each(|attr| {
            if attr.path.is_ident("default") {
                if attributes.default.is_some() {
                    unimplemented!("Duplicated `default` attributes.")
                }
                parse_default(attr, &mut attributes)
            } else if attr.path.is_ident("hidden") {
                attributes.hidden = true;
            } else if attr.path.is_ident("into") {
                attributes.use_into = true
            } else if attr.path.is_ident("validator") {
                parse_validator(attr, &mut attributes)
            } else if attr.path.is_ident("doc") {
                attributes.documents = get_documents(&attrs);
            }
        });
        attributes
    }
}

fn parse_default(attr: &Attribute, attributes: &mut FieldAttributes) {
    attributes.default = Some(attr.parse_args().unwrap());
}

fn parse_validator(attr: &Attribute, attributes: &mut FieldAttributes) {
    attributes.validator = Some(attr.parse_args().unwrap());
}

pub fn get_documents(attrs: &[Attribute]) -> Vec<Attribute> {
    let mut documents: Vec<Attribute> = vec![];

    for attr in attrs {
        if attr.path.is_ident("doc") {
            documents.push(attr.to_owned());
        }
    }

    documents
}
