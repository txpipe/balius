use pallas::{
    ledger::{
        addresses::Address,
        primitives::{
            alonzo::BoundedBytes,
            conway::{BigInt, Constr, ExUnits, PlutusData},
        },
    },
    txbuilder::{BuildBabbage, Output},
};
use rocket::http::Status;
use rocket_dyn_templates::context;

#[macro_use]
extern crate rocket;

/// @ticket_policy = 75b0f5537cd8eca81f95199549233a765883b597b3e5b99c47675e07
/// @vending_machine = addr_test1wrm2dgsajhv8pjasjpkdgts4yftmaa7sjtf90us3yapjkuq8qfs76
/// ^init_state, prev_state, next_state: EventState
/// !params: EventSetup
/// !purchase: TicketPurchase
///
/// tx setup_event:
///     <-* @owner[$ada]
///     <-$ @ticket_policy[!params]
///     --$ $tickets
///     --> @vending_machine[$asset1 + ^init_state]
///
/// tx purchase_ticket:
///     <-* @buyer[$ada]
///     <-- @vending_machine[^prev_state + !purchase]
///     --> @seller[$ada]
///     --> @buyer[$asset1]
///     --> @vending_machine[^next_state + $asset1]

struct EventSetup {
    owner: String,
}

impl From<EventSetup> for PlutusData {
    fn from(value: EventSetup) -> Self {
        PlutusData::Array(vec![PlutusData::Constr(Constr {
            tag: 121,
            any_constructor: None,
            fields: vec![PlutusData::BoundedBytes(BoundedBytes::from(
                value.owner.as_bytes().to_vec(),
            ))],
        })])
    }
}

struct SetupEventTx {
    asset1: (String, i64),
    params: EventSetup,
}

impl SetupEventTx {
    const POLICY1: &'static str = "75b0f5537cd8eca81f95199549233a765883b597b3e5b99c47675e07";
    const SELLER: &'static str = "addr_test1qrmwan3p7l3a6w9n52mz3jqsg6zvw0at9f830t0ur0tmgup3vrgyu8jf0945asdjgaf679x430qpzvpql4khxy2xljeq305yz6";

    fn to_cbor(self) -> String {
        let tx = pallas::txbuilder::StagingTransaction::new()
            .mint_asset(
                SetupEventTx::POLICY1.parse().unwrap(),
                self.asset1.0.as_bytes().to_vec(),
                self.asset1.1,
            )
            .unwrap()
            .output(
                Output::new(Address::from_bech32("addr_test1wrm2dgsajhv8pjasjpkdgts4yftmaa7sjtf90us3yapjkuq8qfs76").unwrap(), 2_000_000).add_asset(SetupEventTx::POLICY1.parse().unwrap(), self.asset1.0.as_bytes().to_vec(), self.asset1.1 as u64).unwrap())
            .add_mint_redeemer(
                SetupEventTx::POLICY1.parse().unwrap(),
                pallas::codec::minicbor::to_vec(PlutusData::from(self.params)).unwrap(),
                Some(pallas::txbuilder::ExUnits {
                    mem: 200,
                    steps: 200,
                }),
            );

        hex::encode(tx.build_babbage_raw().unwrap().tx_bytes)
    }
}

#[get("/setup")]
fn setup_event() -> String {
    let tx1 = SetupEventTx { asset1: ("toctoc".into(), 50), params: EventSetup { owner: "addr_test1qrmwan3p7l3a6w9n52mz3jqsg6zvw0at9f830t0ur0tmgup3vrgyu8jf0945asdjgaf679x430qpzvpql4khxy2xljeq305yz6".into()}};

    tx1.to_cbor()
}

#[get("/sign")]
fn sign() -> rocket_dyn_templates::Template {
    let tx1 = SetupEventTx { 
        asset1: ("toctoc".into(), 50),
        params: EventSetup {
            owner: "addr_test1qrmwan3p7l3a6w9n52mz3jqsg6zvw0at9f830t0ur0tmgup3vrgyu8jf0945asdjgaf679x430qpzvpql4khxy2xljeq305yz6".into()
        }
    };

    rocket_dyn_templates::Template::render(
        "sign",
        context! {
            cbor: tx1.to_cbor(),
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(rocket_dyn_templates::Template::fairing())
        .mount("/", routes![setup_event, sign])
}
