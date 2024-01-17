const BLOCK_INTERVAL: i32 = 5000;
// the block contains the maximum number of transactions
const MAX_BLOCK_SIZE_GLOBAL: i32 = 2000;
// the transaction inject speed
const INJECT_SPEED: i32 = 2000;
// the total number of txs
const TOTAL_DATA_SIZE: i32 = 100000;
// supervisor read a batch of txs then send them, it should be larger than inject speed
const BATCH_SIZE: i32 = 16000;
const BROKER_NUM: i32 = 10;
const NODES_IN_SHARD: i32 = 4;
const SHARD_NUM: i32 = 4;
// measurement data result output path
const DATA_WRITE_PATH: &'static str = "./result/";
// log output path
const LOG_WRITE_PATH: &'static str = "./log";
//supervisor ip address
const SUPERVISOR_ADDR: &'static str = "127.0.0.1:18800";
//the raw BlockTransaction data path
const FILE_INPUT: &'static str = "../ 2000000to2999999_BlockTransaction.csv" ;