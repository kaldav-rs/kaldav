mod filter;
mod object;

#[proc_macro_derive(Object)]
pub fn object_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    object::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn filter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    filter::impl_macro(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
