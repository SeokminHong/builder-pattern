use syn::Attribute;
use syn::Path;
use syn::{Expr, Meta, NestedMeta};

pub struct FieldAttributes {
    pub default: Option<Expr>,
    pub use_into: bool,
}

impl Default for FieldAttributes {
    fn default() -> Self {
        FieldAttributes {
            default: None,
            use_into: false,
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
            } else if attr.path.is_ident("setter") {
                parse_setter(attr, &mut attributes)
            }
        });
        attributes
    }
}

fn parse_default(attr: &Attribute, attributes: &mut FieldAttributes) {
    attributes.default = Some(attr.parse_args().unwrap());
}

fn parse_setter(attr: &Attribute, attributes: &mut FieldAttributes) {
    let mut paths: Vec<Path> = vec![];
    match attr.parse_meta().unwrap() {
        Meta::Path(p) => paths.push(p),
        Meta::List(l) => {
            l.nested.into_iter().for_each(|m| {
                if let NestedMeta::Meta(Meta::Path(p)) = m {
                    paths.push(p)
                }
            });
        }
        Meta::NameValue(_) => {}
    }
    for p in paths.into_iter() {
        if p.is_ident("into") {
            attributes.use_into = true
        }
    }
}
