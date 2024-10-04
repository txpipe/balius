use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function
    let input = parse_macro_input!(item as ItemFn);
    let func_name = &input.sig.ident;
    let func_body = &input.block;

    // Generate the new function with timing
    let output = quote! {
        fn #func_name() -> balius_sdk::Worker {
            #func_body
        }

        struct _Main;

        impl balius_sdk::wit::Guest for _Main {
            fn init(env: balius_sdk::wit::Env) {
                let worker = #func_name();
                balius_sdk::_internal::global_init_worker(env, worker);
            }

            fn handle(
                id: u32,
                evt: balius_sdk::wit::Event,
            ) -> std::result::Result<balius_sdk::wit::Response, balius_sdk::wit::HandleError> {
                balius_sdk::_internal::global_handle_request(id, evt)
            }
        }

        balius_sdk::wit::export!(_Main);
    };

    output.into()
}
