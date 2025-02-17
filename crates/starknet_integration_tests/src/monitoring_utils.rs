use starknet_api::block::BlockNumber;
use starknet_infra_utils::run_until::run_until;
use starknet_infra_utils::tracing::{CustomLogger, TraceLevel};
use starknet_monitoring_endpoint::test_utils::MonitoringClient;
use starknet_sequencer_metrics::metric_definitions;
use tracing::info;

use crate::sequencer_manager::NodeSetup;

/// Gets the latest block number from the batcher's metrics.
pub async fn get_batcher_latest_block_number(
    batcher_monitoring_client: &MonitoringClient,
) -> BlockNumber {
    BlockNumber(
        batcher_monitoring_client
            .get_metric::<u64>(metric_definitions::STORAGE_HEIGHT.get_name())
            .await
            .expect("Failed to get storage height metric."),
    )
    .prev() // The metric is the height marker so we need to subtract 1 to get the latest.
    .expect("Storage height should be at least 1.")
}

/// Sample the metrics until sufficiently many blocks have been reported by the batcher. Returns an
/// error if after the given number of attempts the target block number has not been reached.
pub async fn await_batcher_block(
    interval: u64,
    target_block_number: BlockNumber,
    max_attempts: usize,
    node: &NodeSetup,
) -> Result<BlockNumber, ()> {
    let condition = |&latest_block_number: &BlockNumber| latest_block_number >= target_block_number;
    let get_latest_block_number_closure =
        || get_batcher_latest_block_number(node.batcher_monitoring_client());

    let logger = CustomLogger::new(
        TraceLevel::Info,
        Some(format!(
            "Waiting for batcher height metric to reach block {target_block_number} in sequencer \
             {} executable {}.",
            node.get_node_index().unwrap(),
            node.get_batcher_index()
        )),
    );

    run_until(interval, max_attempts, get_latest_block_number_closure, condition, Some(logger))
        .await
        .ok_or(())
}

pub async fn await_execution(node: &NodeSetup, expected_block_number: BlockNumber) {
    info!(
        "Awaiting until {expected_block_number} blocks have been created in sequencer {}.",
        node.get_node_index().unwrap()
    );
    await_batcher_block(5000, expected_block_number, 50, node)
        .await
        .expect("Block number should have been reached.");
}
