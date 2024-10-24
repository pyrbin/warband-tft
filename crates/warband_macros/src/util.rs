use proc_macro2::{Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse_str, Expr, ExprPath, Ident, Path, PathSegment};

pub(crate) fn ident_from_attr(attr: &syn::Attribute) -> Option<Ident> {
    attr.parse_args::<Path>()
        .ok()
        .and_then(|path| path.get_ident().cloned())
}

pub(crate) fn crate_ident(crate_ident: &str, module_path: &str) -> TokenStream {
    let module_path = parse_str::<Expr>(module_path).unwrap();
    let module_path = match module_path {
        Expr::Path(ExprPath { path, .. }) => path,
        _ => panic!("expected a path"),
    };
    let module_path = module_path
        .segments
        .iter()
        .map(|PathSegment { ident, arguments }| quote!(#ident #arguments))
        .collect::<Vec<_>>();

    match crate_name(crate_ident)
        .unwrap_or_else(|_| panic!("expected {crate_ident:?} is present in `Cargo.toml`"))
    {
        FoundCrate::Itself => {
            quote!(crate::#(#module_path)::*)
        },
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident::#(#module_path)::* )
        },
    }
}
