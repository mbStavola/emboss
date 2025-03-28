use proc_macro2::{Ident, Span};

pub(crate) fn key_const(index: usize) -> Ident {
    Ident::new(&format!("KEY_{}", index), Span::call_site())
}

pub(crate) fn value_const(index: usize) -> Ident {
    Ident::new(&format!("VALUE_{}", index), Span::call_site())
}

pub(crate) fn key_size_const(index: usize) -> Ident {
    Ident::new(&format!("KEY_{}_SIZE", index), Span::call_site())
}

pub(crate) fn value_size_const(index: usize) -> Ident {
    Ident::new(&format!("VALUE_{}_SIZE", index), Span::call_site())
}

pub(crate) fn key_field(index: usize) -> Ident {
    Ident::new(&format!("key_{}", index), Span::call_site())
}

pub(crate) fn value_field(index: usize) -> Ident {
    Ident::new(&format!("value_{}", index), Span::call_site())
}

pub(crate) fn enum_variant(key: &String, variant_name: Option<&String>) -> Option<Ident> {
    variant_name
        .or(Some(key))
        .map(|variant_name| syn::parse_str::<Ident>(variant_name))
        .transpose()
        .ok()
        .flatten()
}
