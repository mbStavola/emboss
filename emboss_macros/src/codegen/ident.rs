use proc_macro2::{Ident, Span};

pub fn key_const(index: usize) -> Ident {
    Ident::new(&format!("KEY_{}", index), Span::call_site())
}

pub fn value_const(index: usize) -> Ident {
    Ident::new(&format!("VALUE_{}", index), Span::call_site())
}

pub fn key_size_const(index: usize) -> Ident {
    Ident::new(&format!("KEY_{}_SIZE", index), Span::call_site())
}

pub fn value_size_const(index: usize) -> Ident {
    Ident::new(&format!("VALUE_{}_SIZE", index), Span::call_site())
}

pub fn key_field(index: usize) -> Ident {
    Ident::new(&format!("key_{}", index), Span::call_site())
}

pub fn value_field(index: usize) -> Ident {
    Ident::new(&format!("value_{}", index), Span::call_site())
}

pub fn enum_variant(key: &String, variant_name: Option<&String>) -> Option<Ident> {
    variant_name
        .or(Some(key))
        .map(|variant_name| syn::parse_str::<Ident>(variant_name))
        .transpose()
        .ok()
        .flatten()
}

pub fn qualified_enum_variant(key: &String, variant_name: Option<&String>) -> Option<Ident> {
    enum_variant(key, variant_name).map(|variant_name| {
        Ident::new(
            &format!("EmbossedVariantKind::{}", variant_name),
            Span::call_site(),
        )
    })
}
