use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdResult, Storage, Uint128};
use cosmwasm_storage::{Bucket, ReadonlyBucket, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";
pub static PREVOTES_KEY: &[u8] = b"prevotes";
pub static VOTES_KEY: &[u8] = b"votes";
pub static STATUS_KEY: &[u8] = b"status";
pub static COUNT_KEY: &[u8] = b"count";
pub static WINNER_KEY: &[u8] = b"winner";
pub static PLAYERS_KEY: &[u8] = b"players";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub price: Uint128,
    pub players: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    PrevoteStage,
    VoteStage,
    Done,
}

pub fn get_players<S: Storage>(storage: &S) -> StdResult<Vec<CanonicalAddr>> {
    ReadonlySingleton::<S, Vec<CanonicalAddr>>::new(storage, PLAYERS_KEY).load()
}

pub fn set_players<S: Storage>(storage: &mut S, players: &Vec<CanonicalAddr>) -> StdResult<()> {
    Singleton::<S, Vec<CanonicalAddr>>::new(storage, PLAYERS_KEY).save(players)
}

pub fn get_winner<S: Storage>(storage: &S) -> StdResult<CanonicalAddr> {
    ReadonlySingleton::new(storage, WINNER_KEY).load()
}

pub fn set_winner<S: Storage>(storage: &mut S, winner: &CanonicalAddr) -> StdResult<()> {
    Singleton::new(storage, WINNER_KEY).save(winner)
}

pub fn get_count<S: Storage>(storage: &S) -> StdResult<u8> {
    ReadonlySingleton::new(storage, COUNT_KEY).load()
}

pub fn set_count<S: Storage>(storage: &mut S, count: u8) -> StdResult<()> {
    Singleton::new(storage, COUNT_KEY).save(&count)
}

pub fn get_config<S: Storage>(storage: &S) -> StdResult<Config> {
    ReadonlySingleton::new(storage, CONFIG_KEY).load()
}

pub fn set_config<S: Storage>(storage: &mut S, config: &Config) -> StdResult<()> {
    Singleton::new(storage, CONFIG_KEY).save(config)
}

pub fn get_status<S: Storage>(storage: &S) -> StdResult<Status> {
    ReadonlySingleton::new(storage, STATUS_KEY).load()
}

pub fn set_status<S: Storage>(storage: &mut S, status: &Status) -> StdResult<()> {
    Singleton::new(storage, STATUS_KEY).save(status)
}

pub fn set_prevote<S: Storage>(
    storage: &mut S,
    address: &CanonicalAddr,
    prevote: &String,
) -> StdResult<()> {
    Bucket::<S, String>::new(PREVOTES_KEY, storage).save(address.as_slice(), prevote)
}

pub fn get_prevote<S: Storage>(storage: &S, address: &CanonicalAddr) -> StdResult<String> {
    ReadonlyBucket::new(PREVOTES_KEY, storage).load(address.as_slice())
}

pub fn set_vote<S: Storage>(
    storage: &mut S,
    address: &CanonicalAddr,
    vote: &String,
) -> StdResult<()> {
    Bucket::<S, String>::new(VOTES_KEY, storage).save(address.as_slice(), vote)
}

pub fn get_vote<S: Storage>(storage: &S, address: &CanonicalAddr) -> StdResult<String> {
    ReadonlyBucket::new(VOTES_KEY, storage).load(address.as_slice())
}
