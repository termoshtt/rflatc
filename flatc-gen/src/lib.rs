extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn flatc_gen(_path: TokenStream) -> TokenStream {
    quote!("").into()
}
