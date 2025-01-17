use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Token,
};

struct Comp {
    mapping: Mapping,
    binding: Binding,
    conditions: Vec<Condition>,
}

impl quote::ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Binding {
            pattern: Pattern(pattern),
            iterable,
        } = &self.binding;
        let Mapping(mapping) = &self.mapping;

        let conditions = &self.conditions;

        tokens.extend(quote! {
            ::core::iter::IntoIterator::into_iter(#iterable).flat_map(move |#pattern| {
                (true #(&& #conditions)*).then(|| #mapping)
            })
        });
    }
}

impl quote::ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mapping = Mapping::parse(input)?;
        input.parse::<Token![in]>()?;
        let binding = Binding::parse(input)?;
        let conditions = parse_zero_or_more(input);

        Ok(Self {
            mapping,
            binding,
            conditions,
        })
    }
}

struct Mapping(syn::Expr);
impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

struct Binding {
    pattern: Pattern,
    iterable: syn::Expr,
}
impl Parse for Binding {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pattern = Pattern::parse(input)?;
        input.parse::<Token![<-]>()?;
        let iterable = input.parse()?;
        Ok(Self { pattern, iterable })
    }
}

struct Pattern(syn::Pat);
impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        syn::Pat::parse_single(input).map(Self)
    }
}

struct Condition(syn::Expr);
impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![,]>()?;
        input.parse().map(Self)
    }
}

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result = Vec::new();
    while let Ok(item) = T::parse(input) {
        result.push(item);
    }
    result
}

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as Comp);
    quote! {#c}.into()
}
