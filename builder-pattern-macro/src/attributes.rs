use bitflags::bitflags;
use syn::{Attribute, Expr, Meta, NestedMeta};

bitflags! {
    pub struct Setters: u32 {
        const VALUE = 0b00000001;
        const LAZY = 0b00000010;
        const ASYNC = 0b00000100;
    }
}

pub struct FieldAttributes {
    pub default: Option<Expr>,
    pub hidden: bool,
    pub use_into: bool,
    pub validator: Option<Expr>,
    pub documents: Vec<Attribute>,
    pub setters: Setters,
}

impl Default for FieldAttributes {
    fn default() -> Self {
        FieldAttributes {
            default: None,
            hidden: false,
            use_into: false,
            validator: None,
            documents: vec![],
            setters: Setters::VALUE,
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
            } else if attr.path.is_ident("setter") {
                parse_setters(attr, &mut attributes)
            }
        });
        match attributes.validate() {
            Ok(_) => attributes,
            Err(e) => unimplemented!("{}", e),
        }
    }
}

fn parse_default(attr: &Attribute, attributes: &mut FieldAttributes) {
    attributes.default = match attr.parse_args() {
        Ok(ex) => Some(ex),
        Err(_) => unimplemented!("Invalid default value."),
    };
}

fn parse_validator(attr: &Attribute, attributes: &mut FieldAttributes) {
    attributes.validator = match attr.parse_args() {
        Ok(ex) => Some(ex),
        Err(_) => unimplemented!("Invalid validator."),
    };
}

fn parse_setters(attr: &Attribute, attributes: &mut FieldAttributes) {
    let meta = attr.parse_meta().unwrap();
    let mut setters = Setters::empty();
    if let Meta::List(l) = meta {
        let it = l.nested.iter();
        it.for_each(|m| {
            if let NestedMeta::Meta(Meta::Path(p)) = m {
                if p.is_ident("value") {
                    setters.insert(Setters::VALUE);
                } else if p.is_ident("lazy") {
                    setters.insert(Setters::LAZY);
                } else if p.is_ident("async") {
                    setters.insert(Setters::ASYNC);
                }
            } else {
                unimplemented!("Invalid setter.")
            }
        });
    } else {
        unimplemented!("Invalid setter.")
    }
    attributes.setters = setters;
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
