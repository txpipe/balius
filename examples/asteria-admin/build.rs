use cargo_metadata::{CargoOpt, MetadataCommand};
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

fn plutus_codegen() -> String {
    let code = quote::quote! {};

    let syntax_tree = syn::parse_file(&code.to_string()).unwrap();
    prettyplease::unparse(&syntax_tree)
}

mod plutus_blueprints {
    pub mod txpipe {
        pub mod asteria {
            use balius_sdk::txbuilder::plutus::{self, IntoData as _};
            use hex_literal::hex;
            use serde::{Deserialize, Serialize};

            type Int = u64;

            pub type PolicyId = Vec<u8>;
            pub type AssetName = Vec<u8>;

            #[serde_with::serde_as]
            #[derive(Debug, Serialize, Deserialize)]
            pub struct AssetClass {
                #[serde_as(as = "serde_with::hex::Hex")]
                policy: PolicyId,
                #[serde_as(as = "serde_with::hex::Hex")]
                name: AssetName,
            }

            impl plutus::IntoData for AssetClass {
                fn into_data(&self) -> plutus::PlutusData {
                    balius_sdk::constr!(0, self.policy, self.name)
                }
            }

            #[derive(Debug, Serialize, Deserialize)]
            pub struct AsteriaSpendParameters {
                pub admin_token: AssetClass,
                pub ship_mint_lovelace_fee: Int,
                pub max_asteria_mining: Int,
            }

            const ASTERIA_SPEND_COMPILED_CODE: &[u8] = &hex!("5907f101010032323232323232223223225333007323232323253323300d3001300e3754004264646464646464a66602860060022a66602e602c6ea8028540085854ccc050c02000454ccc05cc058dd50050a8010b0a99980a18020008a99980b980b1baa00a150021616301437540122646464a66602a6008602c6ea80184c8c8c8c8c8c8c8c94ccc074c030c078dd5000899191919191919192999812980a00389919299981518168010992999814180c18149baa001132330080011323232533302c3375e6012605c6ea8018dd31991912999817980f0160800899191980080080191299981a80089981b19bb04c1014000374c00697adef6c6013232323253330363372091010000213303a337609801014000374c00e00a2a66606c66e3d22100002132533303730263038375400226607666ec1301014000303c30393754002008200864a66606ea66607400229445280a6103d87a80001301d3303b374c00297ae0323300100100222533303b00113303c337609801014000375006a97adef6c60132323232533303c33720910100002133040337609801014000375007200a2a66607866e3d22100002132533303d302c303e375400226608266ec13010140003042303f3754002008200864a66607a6058002298103d87a80001302333041375000297ae03370000207226608066ec0dd48011ba800133006006003375a607a0066eb8c0ec008c0fc008c0f40044cc0e8cdd81ba9002374c0026600c00c0066eacc0dc00cdd7181a801181c801181b800991900118018009981999bb04c01014000375005897adef6c60300100122533303100114984c94ccc0c8004526132325333031337206eb8c0c8c0d8010dd718190010998028029981a8010008b181b001181a000981a0009bab3009302e37546012605c6ea803c54ccc0b000c54ccc0b000840045280a5014a066e3cdd7180418169baa0030133370e6eb4c044c0b0dd500119b8001348008cdc4240006660126eacc018c0acdd50019bae3010302b37540506eb8c018c0acdd5014181698151baa001163005302937540022c605600266002022466ebcc034c0a0dd5000980698141baa300330283754012264a66604c603401026464a666056605c004264a666052603260546ea80044c8cc0240044c8c94ccc0b0c06cc0b4dd5000899191929998178028a99981780108008a5014a066ebc064014c8c8c8cdc499b82375a606a0026eb4c0d4c0d8008cdc11bad3035002375a606a606c00260626ea8c8c8c05ccc0d4dd419b82375a606c0046eb4c0d8004cc0d4dd419b82375a606c606e0046eb4c0d8c0dc0052f5c060646ea8c8c8c8c8c068cc0e0dd419b81337046eb4c0e4010004cdc11bad303900200333038375066e0800c0052f5c06eb4c0e0c0e4004c0d0dd5181b981a1baa007375a606c606e00260646ea8c00d200230313754600460066eacc030c0c4dd5180618189baa01230303754600260046eacc02cc0c0dd50039180a198191ba8001330324c10101004bd701199806800a450048810016533302b4a0298103d87a80001533302b4a0260226605e60226605e6ea0cdc0a400004c6605e6ea120c7014bd7025eb804c044cc0bcc044cc0bcdd401319817a601021864004bd7025eb80c014cc03c05c8c8cc004004c8cc004004c94ccc0b8c088c0bcdd50008a5eb7bdb1804dd5981998181baa0013300e37566014605e6ea8c028c0bcdd500180a9129998188008a5eb804cc0c8c0bcc0cc004cc008008c0d0004894ccc0c000452809929998171919b8f33371890001b8d489045348495000001489045348495000375c606600429444cc00c00c004c0cc004c0b8c0acdd50008b180318151baa00116302c0013300201223375e601c60526ea8004c038c0a4dd5180218149baa00a130013300b013253330273330275333027301630283754601c60526ea8c038c0a4dd5180218149baa00114a029452825113371090001998039bab300430293754600860526ea8004dd7180718149baa026375c600860526ea809852812999813180a98139baa00114a2294088c8cc00400400c894ccc0ac00452f5c026464a666054600a00426605c00466008008002266008008002605e004605a002460526054002460506052605200244a666046602460486ea80084c8c8c8c94ccc0a8c0b40085401458dd7181580098158011bad3029001302537540042c44464a6660486030604a6ea8004520001375a6052604c6ea8004c94ccc090c060c094dd50008a6103d87a80001323300100137566054604e6ea8008894ccc0a4004530103d87a8000132323232533302a337220100042a66605466e3c0200084c040cc0b8dd4000a5eb80530103d87a8000133006006003375a60560066eb8c0a4008c0b4008c0ac004cc01000c00888c8cc00400400c894ccc098004530103d87a800013232323253330273372200e0042a66604e66e3c01c0084c034cc0acdd3000a5eb80530103d87a8000133006006003375660500066eb8c098008c0a8008c0a0004c07cdd500a1811180f9baa001163300100923375e6008603e6ea800403c88c8cc00400400c894ccc088004530103d87a8000132325333021300500213007330250024bd70099802002000981300118120009ba5480008c07c004dd7180e980f0011bad301c001301837540026034602e6ea801858dd6180c980d180d0011bac3018001301437540146e1d2000370e9002180a180a801180980098079baa002370e90010b1808180880118078009807801180680098049baa00114984d958dd68009bad0015734aae7555cf2ab9f5740ae855d101");

            pub struct AsteriaSpendValidator;

            impl AsteriaSpendValidator {
                pub fn compiled_code() -> &'static [u8] {
                    &ASTERIA_SPEND_COMPILED_CODE
                }

                pub fn apply_params(params: AsteriaSpendParameters) -> Vec<u8> {
                    let mut cbor_buffer = Vec::new();
                    let mut flat_buffer = Vec::new();

                    let program = uplc::ast::Program::<uplc::ast::DeBruijn>::from_hex(
                        &hex::encode(ASTERIA_SPEND_COMPILED_CODE),
                        &mut cbor_buffer,
                        &mut flat_buffer,
                    )
                    .unwrap();

                    let program = program.apply_data(params.admin_token.into_data());
                    let program = program.apply_data(params.max_asteria_mining.into_data());
                    let program = program.apply_data(params.ship_mint_lovelace_fee.into_data());

                    program.to_cbor().unwrap()
                }
            }

            pub struct AsteriaSpendDatum {}
        }
    }
}

mod telchar {
    use balius_sdk::txbuilder::primitives::PlutusScript;
    use balius_sdk::txbuilder::{MinUtxoLovelace, OutputBuilder, TxBuilder, UtxoSource};
}

#[derive(Debug, Serialize, Deserialize)]
struct TelcharMetadata {
    validators: Vec<ValidatorMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ValidatorMetadata {
    blueprint: String,
    apply_parameters: serde_json::Value,
    deploy_address: Option<String>,
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=plutus.json");

    //// if we wanted to query metadata from the package, we could do this:
    // let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    // let package = metadata
    //     .packages
    //     .iter()
    //     .find(|p| p.name == env!("CARGO_PKG_NAME"))
    //     .expect("Could not find the package");

    let metadata: TelcharMetadata = {
        let content = std::fs::read_to_string("telchar.toml").expect("Failed to read telchar.toml");
        toml::from_str(&content).expect("Failed to parse telchar.toml")
    };

    dbg!(&metadata);

    for validator in metadata.validators {
        let params = serde_json::from_value(validator.apply_parameters).unwrap();
        match validator.blueprint.as_str() {
            "asteria.asteria.spend" => {
                let applied =
                    plutus_blueprints::txpipe::asteria::AsteriaSpendValidator::apply_params(params);
                std::fs::write("asteria.asteria.spend.uplc", hex::encode(applied)).unwrap();
            }
            _ => todo!(),
        }
    }

    let script_bytes = {
        let path = std::path::Path::new("asteria.asteria.spend.uplc");
        std::fs::read(path).expect("Failed to read validator script file")
    };

    // Get the output directory from cargo
    //let out_dir = std::env::var("OUT_DIR").unwrap();
    //let dest_path = Path::new(&out_dir).join("generated.rs");
    //let dest_path = std::path::Path::new("generated.rs");

    // Generate your Rust code here based on the AST
    //let generated_code = plutus_codegen();

    // Write the generated code to a file
}
