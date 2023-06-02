use near_chain_configs::{Genesis, GenesisValidationMode};

#[test]
fn test_load_genesis() {
    println!("SLAVKO DARKO TVRLE");
    let mirko = Genesis::from_file("res/mainnet_genesis.json", GenesisValidationMode::Full).unwrap();
    println!("verzija ovo ono {:?}", mirko.config.protocol_version);
}
