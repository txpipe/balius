use opentelemetry::{
    global,
    metrics::{Counter, Gauge, Histogram},
    KeyValue,
};

use crate::{logging::level_to_string, wit::balius::app::logging::Level};

#[derive(Clone)]
pub struct Metrics {
    requests: Counter<u64>,
    kv_get: Counter<u64>,
    kv_set: Counter<u64>,
    kv_list: Counter<u64>,
    log: Counter<u64>,
    utxo_handled: Counter<u64>,
    tx_handled: Counter<u64>,
    undo_utxo_handled: Counter<u64>,
    undo_tx_handled: Counter<u64>,
    submit_tx: Counter<u64>,
    signer_sign_payload: Counter<u64>,
    ledger_read_utxos: Counter<u64>,
    ledger_search_utxos: Counter<u64>,
    ledger_read_params: Counter<u64>,
    workers_loaded: Gauge<u64>,
    handle_chain_duration_ms: Histogram<f64>,
    handle_request_duration_ms: Histogram<f64>,
    handle_worker_chain_duration_ms: Histogram<f64>,
    latest_block_height: Gauge<u64>,
    latest_block_slot: Gauge<u64>,
}

impl Metrics {
    pub fn new() -> Self {
        let meter = global::meter("balius-runtime");

        let requests = meter
            .u64_counter("requests")
            .with_description("Total number of requests handled.")
            .build();

        let kv_get = meter
            .u64_counter("kv_get")
            .with_description("Total amount of kv get calls.")
            .build();

        let kv_set = meter
            .u64_counter("kv_set")
            .with_description("Total amount of kv set calls.")
            .build();

        let kv_list = meter
            .u64_counter("kv_list")
            .with_description("Total amount of kv list calls.")
            .build();

        let log = meter
            .u64_counter("log")
            .with_description("Total amount of log lines written.")
            .build();

        let utxo_handled = meter
            .u64_counter("utxo_handled")
            .with_description("Amount of UTxO event handled per worker.")
            .build();

        let tx_handled = meter
            .u64_counter("tx_handled")
            .with_description("Amount of Tx event handled per worker.")
            .build();

        let undo_utxo_handled = meter
            .u64_counter("undo_utxo_handled")
            .with_description("Amount of undo UTxO event handled per worker.")
            .build();

        let undo_tx_handled = meter
            .u64_counter("undo_tx_handled")
            .with_description("Amount of undo Tx event handled per worker.")
            .build();

        let submit_tx = meter
            .u64_counter("submit_tx")
            .with_description("Amount of submit_tx calls per worker.")
            .build();

        let signer_sign_payload = meter
            .u64_counter("signer_sign_payload")
            .with_description("Amount of sign payload handled per worker.")
            .build();

        let ledger_read_utxos = meter
            .u64_counter("ledger_read_utxos")
            .with_description("Amount of calls to read_utxos on the ledger interface.")
            .build();

        let ledger_search_utxos = meter
            .u64_counter("ledger_search_utxos")
            .with_description("Amount of calls to search_utxos on the ledger interface.")
            .build();

        let ledger_read_params = meter
            .u64_counter("ledger_read_params")
            .with_description("Amount of calls to read_params on the ledger interface.")
            .build();

        let workers_loaded = meter
            .u64_gauge("workers_loaded")
            .with_description("Current amount of workers loaded into the runtime.")
            .build();

        let handle_chain_duration_ms = meter
            .f64_histogram("handle_chain_duration_ms")
            .with_description("Duration to process handle_chain in milliseconds.")
            .with_unit("ms")
            .with_boundaries(vec![
                100.0, 250.0, 500.0, 1000.0, 2500.0, 5000.0, 10000.0, 25000.0, 60000.0, 1200000.0,
            ])
            .build();

        let handle_request_duration_ms = meter
            .f64_histogram("handle_request_duration_ms")
            .with_description("Duration to process handle_request in milliseconds.")
            .with_unit("ms")
            .with_boundaries(vec![
                100.0, 250.0, 500.0, 1000.0, 2500.0, 5000.0, 10000.0, 25000.0, 60000.0, 1200000.0,
            ])
            .build();

        let handle_worker_chain_duration_ms = meter
            .f64_histogram("handle_worker_chain_duration_ms")
            .with_description("Duration for a worker to process apply_chain in milliseconds.")
            .with_unit("ms")
            .with_boundaries(vec![
                100.0, 250.0, 500.0, 1000.0, 2500.0, 5000.0, 10000.0, 25000.0, 60000.0, 1200000.0,
            ])
            .build();

        let latest_block_height = meter
            .u64_gauge("latest_block_height")
            .with_description("Latest block height successfully processed by handle_chain.")
            .build();

        let latest_block_slot = meter
            .u64_gauge("latest_block_slot")
            .with_description("Latest block slot successfully processed by handle_chain.")
            .build();

        Metrics {
            requests,
            kv_get,
            kv_set,
            kv_list,
            log,
            utxo_handled,
            tx_handled,
            undo_utxo_handled,
            undo_tx_handled,
            submit_tx,
            signer_sign_payload,
            ledger_read_utxos,
            ledger_search_utxos,
            ledger_read_params,
            workers_loaded,
            handle_chain_duration_ms,
            handle_request_duration_ms,
            handle_worker_chain_duration_ms,
            latest_block_height,
            latest_block_slot,
        }
    }

    pub fn request(&self, worker_id: &str, method: &str, success: bool) {
        self.requests.add(
            1,
            &[
                KeyValue::new("worker", worker_id.to_owned()),
                KeyValue::new("method", method.to_owned()),
                KeyValue::new("success", success.to_string()),
            ],
        );
    }

    pub fn kv_get(&self, worker_id: &str) {
        self.kv_get
            .add(1, &[KeyValue::new("worker", worker_id.to_owned())]);
    }

    pub fn kv_set(&self, worker_id: &str) {
        self.kv_set
            .add(1, &[KeyValue::new("worker", worker_id.to_owned())]);
    }

    pub fn kv_list(&self, worker_id: &str) {
        self.kv_list
            .add(1, &[KeyValue::new("worker", worker_id.to_owned())]);
    }

    pub fn log(&self, worker_id: &str, level: &Level) {
        self.log.add(
            1,
            &[
                KeyValue::new("worker", worker_id.to_owned()),
                KeyValue::new("level", level_to_string(level)),
            ],
        );
    }

    pub fn utxo_handled(&self, worker_id: &str) {
        self.utxo_handled
            .add(1, &[KeyValue::new("worker", worker_id.to_owned())]);
    }

    pub fn tx_handled(&self, worker_id: &str) {
        self.tx_handled
            .add(1, &[KeyValue::new("worker", worker_id.to_owned())]);
    }

    pub fn undo_utxo_handled(&self, worker_id: &str) {
        self.undo_utxo_handled
            .add(1, &[KeyValue::new("worker", worker_id.to_owned())]);
    }

    pub fn undo_tx_handled(&self, worker_id: &str) {
        self.undo_tx_handled
            .add(1, &[KeyValue::new("worker", worker_id.to_owned())]);
    }

    pub fn submit_tx(&self, worker_id: &str) {
        self.submit_tx
            .add(1, &[KeyValue::new("worker", worker_id.to_owned())]);
    }
    pub fn signer_sign_payload(&self, worker_id: &str) {
        self.signer_sign_payload
            .add(1, &[KeyValue::new("worker", worker_id.to_owned())]);
    }

    pub fn ledger_read_utxos(&self, worker_id: &str) {
        self.ledger_read_utxos
            .add(1, &[KeyValue::new("worker", worker_id.to_owned())]);
    }

    pub fn ledger_search_utxos(&self, worker_id: &str) {
        self.ledger_search_utxos
            .add(1, &[KeyValue::new("worker", worker_id.to_owned())]);
    }

    pub fn ledger_read_params(&self, worker_id: &str) {
        self.ledger_read_params
            .add(1, &[KeyValue::new("worker", worker_id.to_owned())]);
    }

    pub fn workers_loaded(&self, count: u64) {
        self.workers_loaded.record(count, &[]);
    }

    pub fn handle_chain_duration_ms(&self, duration_ms: f64) {
        self.handle_chain_duration_ms.record(duration_ms, &[]);
    }

    pub fn handle_request_duration_ms(&self, worker_id: &str, method: &str, duration_ms: f64) {
        self.handle_request_duration_ms.record(
            duration_ms,
            &[
                KeyValue::new("worker", worker_id.to_owned()),
                KeyValue::new("method", method.to_owned()),
            ],
        );
    }

    pub fn latest_block_height(&self, height: u64) {
        self.latest_block_height.record(height, &[]);
    }

    pub fn latest_block_slot(&self, slot: u64) {
        self.latest_block_slot.record(slot, &[]);
    }

    pub fn handle_worker_chain_duration_ms(&self, worker_id: &str, duration_ms: f64) {
        self.handle_worker_chain_duration_ms.record(
            duration_ms,
            &[KeyValue::new("worker", worker_id.to_owned())],
        );
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
