use syn::{Attribute, Expr};

pub struct FieldAttributes {
    pub default: Option<Expr>,
    pub use_into: bool,
    pub validator: Option<Expr>,
}

impl Default for FieldAttributes {
    fn default() -> Self {
        FieldAttributes {
            default: None,
            use_into: false,
            validator: None,
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
            } else if attr.path.is_ident("into") {
                attributes.use_into = true
            } else if attr.path.is_ident("validator") {
                parse_validator(attr, &mut attributes)
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
