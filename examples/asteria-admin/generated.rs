mod plutus_blueprints {
    pub mod txpipe {
        pub mod asteria {
            use hex_literal::hex;
            use serde::{Deserialize, Serialize};
            type Int = u64;
            pub struct AsteriaSpendParameters {
                pub admin_token: AssetClass,
                pub ship_mint_lovelace_fee: Int,
                pub max_asteria_mining: Int,
            }
            const ASTERIA_SPEND_COMPILED_CODE: &[u8] = &hex!("abcd");
            pub struct AsteriaSpendValidator;
            impl AsteriaSpendValidator {
                pub fn compiled_code() -> &'static [u8] {
                    &ASTERIA_SPEND_COMPILED_CODE
                }
                pub fn apply_params(params: AsteriaSpendParameters) -> &'static [u8] {
                    todo!()
                }
            }
            pub struct AsteriaSpendDatum {}
            #[derive(Serialize, Deserialize)]
            pub struct AssetClass {}
        }
    }
}
