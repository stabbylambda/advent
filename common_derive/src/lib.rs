use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta};

// Test case we're targeting:
// #[derive(GridTile)]
// enum Tile {
//     #[tile('@')]
//     Roll,
//     #[tile('.')]
//     Empty,
// }

/// Extract the character from a #[tile('c')] attribute
fn extract_tile_char(variant: &syn::Variant) -> Result<char, syn::Error> {
    // Find the #[tile(...)] attribute
    for attr in &variant.attrs {
        if attr.path().is_ident("tile") {
            // Parse the attribute as Meta::List
            let meta = &attr.meta;

            // Handle #[tile('c')] format
            if let Meta::List(meta_list) = meta {
                // Parse the tokens inside the parentheses
                let tokens = &meta_list.tokens;
                let lit: Lit = syn::parse2(tokens.clone())?;

                if let Lit::Char(lit_char) = lit {
                    return Ok(lit_char.value());
                } else {
                    return Err(syn::Error::new_spanned(
                        lit,
                        "tile attribute must contain a character literal"
                    ));
                }
            }
        }
    }

    Err(syn::Error::new_spanned(
        variant,
        "missing #[tile('c')] attribute"
    ))
}

#[proc_macro_derive(GridTile, attributes(tile))]
pub fn derive_grid_tile(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    // Ensure input is an enum
    let data_enum = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => {
            return syn::Error::new_spanned(
                &input,
                "GridTile can only be derived for enums"
            )
            .to_compile_error()
            .into();
        }
    };

    // Collect (variant_name, character) pairs
    let mut variant_chars = Vec::new();

    for variant in &data_enum.variants {
        // Ensure variant has no fields (only unit variants supported)
        match &variant.fields {
            Fields::Unit => {}
            _ => {
                return syn::Error::new_spanned(
                    variant,
                    "GridTile only supports unit variants (no fields)"
                )
                .to_compile_error()
                .into();
            }
        }

        // Extract the tile character
        let tile_char = match extract_tile_char(variant) {
            Ok(c) => c,
            Err(e) => return e.to_compile_error().into(),
        };

        variant_chars.push((&variant.ident, tile_char));
    }

    // Generate Debug implementation
    let debug_arms = variant_chars.iter().map(|(variant_name, tile_char)| {
        quote! {
            Self::#variant_name => write!(f, "{}", #tile_char),
        }
    });

    let debug_impl = quote! {
        impl std::fmt::Debug for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#debug_arms)*
                }
            }
        }
    };

    // Generate Display implementation
    let display_arms = variant_chars.iter().map(|(variant_name, tile_char)| {
        quote! {
            Self::#variant_name => write!(f, "{}", #tile_char),
        }
    });

    let display_impl = quote! {
        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms)*
                }
            }
        }
    };

    let expanded = quote! {
        #debug_impl
        #display_impl
    };

    TokenStream::from(expanded)
}
