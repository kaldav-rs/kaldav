fn kaldav() -> proc_macro2::TokenStream {
    match (
        proc_macro_crate::crate_name("kaldav"),
        std::env::var("CARGO_CRATE_NAME").as_deref(),
    ) {
        (Ok(proc_macro_crate::FoundCrate::Itself), Ok("kaldav")) => quote::quote!(crate),
        (Ok(proc_macro_crate::FoundCrate::Name(name)), _) => {
            let ident = proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
            quote::quote!(::#ident)
        }
        _ => quote::quote!(::kaldav),
    }
}

pub(crate) fn impl_macro(ast: &Map) -> syn::Result<proc_macro2::TokenStream> {
    let body = quote::quote! {
        #ast
    };

    Ok(body)
}

#[derive(Clone, Debug)]
pub(crate) struct Map(Filter);

impl syn::parse::Parse for Map {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

impl quote::ToTokens for Map {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let kaldav = kaldav();
        let filter = &self.0;

        let body = quote::quote! {{
            use #kaldav::elements::*;
            use #kaldav::elements::filter::*;

            Filter::new()
            #filter
        }};

        tokens.extend(body);
    }
}

#[derive(Clone, Debug)]
struct Filter {
    name: syn::Ident,
    me: Arg,
    children: Vec<Self>,
}

impl syn::parse::Parse for Filter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = if input.peek2(syn::Token![:]) && input.peek3(syn::Token![:]) {
            proc_macro2::Ident::new("append", proc_macro2::Span::call_site())
        } else {
            let name = input.parse::<syn::Ident>()?;

            input.parse::<syn::Token![:]>()?;

            name
        };

        let mut filter = Self {
            name,
            me: input.parse()?,
            children: Vec::new(),
        };

        if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);

            if !content.is_empty() {
                filter.children.push(content.parse()?);
            }
        }

        input.parse::<syn::Token![,]>().ok();

        Ok(filter)
    }
}

impl quote::ToTokens for Filter {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let me = &self.me;
        let args = &self.children;

        let body = quote::quote! {
            .#name(
                #me
                #( #args )*
            )
        };

        tokens.extend(body);
    }
}

#[derive(Clone, Debug)]
enum Arg {
    Function {
        ty: syn::Path,
        arg: proc_macro2::TokenStream,
    },
    Lit(syn::Lit),
    Ident(syn::Path),
    Struct {
        ty: syn::Ident,
        body: proc_macro2::TokenStream,
    },
}

impl syn::parse::Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let arg = if input.peek(syn::Lit) {
            Self::Lit(input.parse::<syn::Lit>()?)
        } else if input.peek2(syn::token::Brace) {
            let ty = input.parse()?;

            let content;

            syn::braced!(content in input);

            Self::Struct {
                ty,
                body: content.parse()?,
            }
        } else {
            let ty = input.parse()?;

            if input.peek(syn::token::Paren) {
                let content;
                syn::parenthesized!(content in input);

                Self::Function {
                    ty,
                    arg: content.parse()?,
                }
            } else {
                Self::Ident(ty)
            }
        };

        Ok(arg)
    }
}

impl quote::ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let body = match self {
            Self::Function { ty, arg } => quote::quote! {
                #ty(#arg)
            },
            Self::Lit(lit) => quote::quote! { #lit },
            Self::Ident(ident) => quote::quote! { #ident },
            Self::Struct { ty, body } => quote::quote! { #ty { #body } },
        };

        tokens.extend(body);
    }
}
