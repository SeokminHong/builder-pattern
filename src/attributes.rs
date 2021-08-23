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
        match attributes.validate() {
            Ok(_) => attributes,
            Err(e) => unimplemented!("{}", e),
        }
    }
}

fn parse_default(attr: &Attribute, attributes: &mut FieldAttributes) {
    attributes.default = attr.parse_args().ok();
}

fn parse_validator(attr: &Attribute, attributes: &mut FieldAttributes) {
    attributes.validator = attr.parse_args().ok();
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

impl FieldAttributes {
    fn validate(&self) -> Result<(), String> {
        if self.hidden && self.default.is_none() {
            Err(String::from(
                "`hidden` attribute requires `default` attribute.",
            ))
        } else {
            Ok(())
        }
    }
}
