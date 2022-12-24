use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemFn);

    let name = input.sig.ident;
    let block = input.block;

    let expanded = quote! {
        #[no_mangle]
        pub fn #name() {
            executor::run(async move {
                #block
            })
        }
    };

    TokenStream::from(expanded)
}
