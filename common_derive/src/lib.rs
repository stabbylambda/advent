use proc_macro::TokenStream;

#[proc_macro_derive(GridTile, attributes(tile))]
pub fn derive_grid_tile(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
