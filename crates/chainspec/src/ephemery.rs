//! Ephemery testnet support
//!
//! Ephemery is a self-resetting testnet that regenerates every 28 days.
//! Chain ID and genesis timestamp are derived from calendar time.
//! EIP-6916: <https://eips.ethereum.org/EIPS/eip-6916>
//! Genesis parameters: <https://github.com/ephemery-testnet/ephemery-genesis/blob/master/values.env>

use alloy_eips::{eip7840::BlobParams, eip7892::BlobScheduleBlobParams};
use alloy_genesis::Genesis;
use alloy_primitives::U256;
use reth_ethereum_forks::{ChainHardforks, EthereumHardfork, ForkCondition, Hardfork};

/// Ephemery's anchor genesis timestamp (iteration 0)
pub(crate) const EPHEMERY_GENESIS_ANCHOR: u64 = 1393527600;

/// Period length in seconds (28 days)
pub(crate) const PERIOD_IN_SECONDS: u64 = 28 * 24 * 60 * 60;

/// Base chain ID for Ephemery (iteration 0's chain ID)
pub(crate) const EPHEMERY_BASE_CHAIN_ID: u64 = 39438000;

/// BPO1 activation offset from genesis timestamp
pub(crate) const BPO1_OFFSET: u64 = 787032;

/// BPO2 activation offset from genesis timestamp
pub(crate) const BPO2_OFFSET: u64 = 1573464;

/// Computes the current Ephemery period from wall-clock time.
/// Returns (period, chain_id, genesis_timestamp).
#[cfg(feature = "std")]
pub(crate) fn current_ephemery_period() -> (u64, u64, u64) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX epoch")
        .as_secs();

    let period = (now - EPHEMERY_GENESIS_ANCHOR) / PERIOD_IN_SECONDS;
    let chain_id = EPHEMERY_BASE_CHAIN_ID + period;
    let genesis_timestamp = EPHEMERY_GENESIS_ANCHOR + (period * PERIOD_IN_SECONDS);

    (period, chain_id, genesis_timestamp)
}

/// Patches the genesis with the current period's timestamp.
#[cfg(feature = "std")]
pub(crate) fn ephemery_genesis(mut genesis: Genesis) -> Genesis {
    let (_, _, genesis_timestamp) = current_ephemery_period();
    genesis.timestamp = genesis_timestamp;
    genesis
}

/// Builds the hardfork schedule for Ephemery.
///
/// All pre-merge forks activate at block 0, all post-merge forks
/// activate at genesis timestamp (0 offset), except bpo forks
/// which are genesis-relative offsets.
pub(crate) fn ephemery_hardforks(genesis_timestamp: u64) -> ChainHardforks {
    ChainHardforks::new(vec![
        (EthereumHardfork::Frontier.boxed(), ForkCondition::Block(0)),
        (EthereumHardfork::Homestead.boxed(), ForkCondition::Block(0)),
        (EthereumHardfork::Tangerine.boxed(), ForkCondition::Block(0)),
        (EthereumHardfork::SpuriousDragon.boxed(), ForkCondition::Block(0)),
        (EthereumHardfork::Byzantium.boxed(), ForkCondition::Block(0)),
        (EthereumHardfork::Constantinople.boxed(), ForkCondition::Block(0)),
        (EthereumHardfork::Petersburg.boxed(), ForkCondition::Block(0)),
        (EthereumHardfork::Istanbul.boxed(), ForkCondition::Block(0)),
        (EthereumHardfork::Berlin.boxed(), ForkCondition::Block(0)),
        (EthereumHardfork::London.boxed(), ForkCondition::Block(0)),
        (
            EthereumHardfork::Paris.boxed(),
            ForkCondition::TTD {
                activation_block_number: 0,
                total_difficulty: U256::ZERO,
                fork_block: None,
            },
        ),
        (EthereumHardfork::Shanghai.boxed(), ForkCondition::Timestamp(0)),
        (EthereumHardfork::Cancun.boxed(), ForkCondition::Timestamp(0)),
        (EthereumHardfork::Prague.boxed(), ForkCondition::Timestamp(0)),
        (EthereumHardfork::Osaka.boxed(), ForkCondition::Timestamp(0)),
        (EthereumHardfork::Amsterdam.boxed(), ForkCondition::Timestamp(0)),
        (EthereumHardfork::Bpo1.boxed(), ForkCondition::Timestamp(genesis_timestamp + BPO1_OFFSET)),
        (EthereumHardfork::Bpo2.boxed(), ForkCondition::Timestamp(genesis_timestamp + BPO2_OFFSET)),
    ])
}

/// Builds the blob params schedule for Ephemery.
pub(crate) fn ephemery_blob_params(genesis_timestamp: u64) -> BlobScheduleBlobParams {
    BlobScheduleBlobParams::default().with_scheduled([
        (
            genesis_timestamp + BPO1_OFFSET,
            BlobParams {
                target_blob_count: 8,
                max_blob_count: 12,
                update_fraction: 6676955,
                ..BlobParams::cancun()
            },
        ),
        (
            genesis_timestamp + BPO2_OFFSET,
            BlobParams {
                target_blob_count: 10,
                max_blob_count: 15,
                update_fraction: 8346193,
                ..BlobParams::cancun()
            },
        ),
    ])
}
