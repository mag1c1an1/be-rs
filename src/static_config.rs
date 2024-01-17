use std::collections::HashMap;
use num_bigint::BigInt;
use once_cell::sync::Lazy;


struct ChainConfig {
    chain_id: u64,
    node_id: u64,
    shard_id: u64,
    nodes_per_shard: u64,
    shard_nums: u64,
    block_size: u64,
    inject_speed: u64,

    // used in transaction relaying, useless in brokerchain mechanism
    max_relay_block_size: u64,
}

const DECIDER_SHARD: u64 = 0xffffffff;
static INIT_BALANCE: Lazy<BigInt> = Lazy::new(|| {
    BigInt::parse_bytes(b"100000000000000000000000000000000000000000000", 10).unwrap()
});

static IP_MAP_NODE_TABLE: Lazy<HashMap<u64, HashMap<u64, String>>> = Lazy::new(|| {
    HashMap::new()
});

const COMMITTEE_METHOD: [&'static str; 4] = ["CLPA_Broker", "CLPA", "Broker", "Relay"];
const MEASURE_BROKER_MOD: [&'static str; 4] = ["TPS_Broker", "TCL_Broker", "CrossTxRate_Broker", "TxNumberCount_Broker"];
const MEASURE_RELAY_MOD: [&'static str; 4] = ["TPS_Relay", "TCL_Relay", "CrossTxRate_Relay", "TxNumberCount_Relay"];

#[test]
fn test_bigint() {
    println!("{}",*INIT_BALANCE);
}