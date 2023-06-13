use near_config_utils::ValidationError;
use near_primitives::shard_layout::ShardLayout;
use near_primitives::{
    serialize::dec_format,
    types::{
        AccountId, AccountInfo, Balance, BlockHeightDelta, Gas, NumBlocks, NumSeats
    },
};
use num_rational::Rational32;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenesisConfigPatch {
    pub num_block_producer_seats: Option<NumSeats>,
    pub num_block_producer_seats_per_shard: Option<Vec<NumSeats>>,
    pub avg_hidden_validator_seats_per_shard: Option<Vec<NumSeats>>,
    pub dynamic_resharding: Option<bool>,
    pub protocol_upgrade_stake_threshold: Option<Rational32>,
    pub epoch_length: Option<BlockHeightDelta>,
    pub gas_limit: Option<Gas>,
    #[serde(with = "dec_format")]
    #[serde(default)]
    pub min_gas_price: Option<Balance>,
    #[serde(with = "dec_format")]
    #[serde(default)]
    pub max_gas_price: Option<Balance>,
    pub block_producer_kickout_threshold: Option<u8>,
    pub chunk_producer_kickout_threshold: Option<u8>,
    pub online_min_threshold: Option<Rational32>,
    pub online_max_threshold: Option<Rational32>,
    pub gas_price_adjustment_rate: Option<Rational32>,
    pub validators: Option<Vec<AccountInfo>>,
    pub transaction_validity_period: Option<NumBlocks>,
    pub protocol_reward_rate: Option<Rational32>,
    pub max_inflation_rate: Option<Rational32>,
    #[serde(with = "dec_format")]
    #[serde(default)]
    pub total_supply: Option<Balance>,
    pub num_blocks_per_year: Option<NumBlocks>,
    pub protocol_treasury_account: Option<AccountId>,
    #[serde(with = "dec_format")]
    #[serde(default)]
    pub fishermen_threshold: Option<Balance>,
    pub minimum_stake_divisor: Option<u64>,
    pub shard_layout: Option<ShardLayout>,
    pub num_chunk_only_producer_seats: Option<NumSeats>,
    pub minimum_validators_per_shard: Option<NumSeats>,
    pub max_kickout_stake_perc: Option<u8>,
    pub minimum_stake_ratio: Option<Rational32>,
}

impl GenesisConfigPatch {
    pub fn from_file_patch<P: AsRef<Path>>(path: P) -> Result<GenesisConfigPatch, ValidationError> {
        let mut file = File::open(&path).map_err(|_| ValidationError::GenesisFileError {
            error_message: format!(
                "Could not open genesis patch config file at path {}.",
                &path.as_ref().display()
            ),
        })?;

        let mut json_str = String::new();
        file.read_to_string(&mut json_str).map_err(|_| ValidationError::GenesisFileError {
            error_message: "Failed to read genesis patch config file to string. ".to_string(),
        })?;

        let json_str_without_comments = near_config_utils::strip_comments_from_json_str(&json_str)
            .map_err(|_| ValidationError::GenesisFileError {
                error_message: "Failed to strip comments from genesis patch config file".to_string(),
            })?;

        let genesis_patch = serde_json::from_str::<GenesisConfigPatch>(&json_str_without_comments)
            .map_err(|_| ValidationError::GenesisFileError {
                error_message: "Failed to deserialize the genesis patch records.".to_string(),
            })?;

        Ok(genesis_patch)
    }
}
