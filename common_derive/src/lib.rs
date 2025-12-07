use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

// Test case we're targeting:
// #[derive(GridTile)]
// enum Tile {
//     #[tile('@')]
//     Roll,
//     #[tile('.')]
//     Empty,
// }

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

    TokenStream::new()
}
