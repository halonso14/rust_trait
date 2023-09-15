use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(From)]
pub fn derive_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, .. } = input;

    let output = quote! {
        impl From<#ident> for SuccessResponse {
            quote! {
                fn from(value: #ident) -> Self {
                    SuccessResponse {
                        error: 0,
                        success: value.error,
                    }
                }
            }
        }

        impl From<#ident> for FailuerResponse {
            quote! {
                fn from(value: #ident) -> Self {
                    FailuerResponse {
                        error: 0,
                        success: value.error,
                    }
                }
            }
        }
    };

    output.into()
}
