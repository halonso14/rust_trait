use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(From)]
pub fn derive_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, .. } = input;

    let success_token_stream = quote! {
        fn from(value: #ident) -> Self {
            SuccessResponse {
                error: 0,
                success: value.error,
            }
        }
    };

    let failure_token_stream = quote! {
        fn from(value: #ident) -> Self {
            FailuerResponse {
                error: 0,
                success: value.failure,
            }
        }
    };

    let output = quote! {
        impl From<#ident> for SuccessResponse {
            #success_token_stream
        }

        impl From<#ident> for FailuerResponse {
            #failure_token_stream
        }
    };

    output.into()
}
