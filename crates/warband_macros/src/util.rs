use syn::{Ident, Path};

pub(crate) fn ident_from_attr(attr: &syn::Attribute) -> Option<Ident> {
    attr.parse_args::<Path>()
        .ok()
        .and_then(|path| path.get_ident().cloned())
}
