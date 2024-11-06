use pallas::codec::minicbor::decode::info;
use serde::{Deserialize, Serialize};
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};
use utxorpc::CardanoSyncClient;

use crate::{ChainPoint, Error, Runtime};

impl From<ChainPoint> for utxorpc::spec::sync::BlockRef {
    fn from(point: ChainPoint) -> Self {
        utxorpc::spec::sync::BlockRef {
            index: point.0,
            hash: point.1.to_vec().into(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub endpoint_url: String,
    pub api_key: String,
}

pub async fn run(config: Config, runtime: Runtime, cancel: CancellationToken) -> Result<(), Error> {
    let mut sync = utxorpc::ClientBuilder::new()
        .uri(&config.endpoint_url)?
        .metadata("dmtr-api-key", config.api_key)?
        .build::<CardanoSyncClient>()
        .await;

    let cursor = runtime
        .chain_cursor()
        .await?
        .map(Into::into)
        .into_iter()
        .collect();

    // TODO: handle disconnections and retry logic

    let mut tip = sync.follow_tip(cursor).await?;

    info!("starting follow-tip loop");

    loop {
        select! {
            _ = cancel.cancelled() => {
                warn!("chainsync driver cancelled");
                break Ok(())
            },
            event = tip.event() => {
                match event {
                    Ok(utxorpc::TipEvent::Apply(block)) => {
                        let block = pallas::ledger::traverse::MultiEraBlock::decode(&block.native).unwrap();
                        runtime.apply_block(&block).await?;
                    }
                    Ok(utxorpc::TipEvent::Undo(block)) => {
                        let block = pallas::ledger::traverse::MultiEraBlock::decode(&block.native).unwrap();
                        runtime.undo_block(&block).await?;
                    }
                    Ok(utxorpc::TipEvent::Reset(point)) => {
                        warn!(slot=point.index, "TODO: handle reset");
                        continue;
                    },
                    Err(_) => todo!(),
                }
            }
        }
    }
}
