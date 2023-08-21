use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, StdResult, Storage, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct bet {
    pub prediction: Prediction,
    pub amount: Uint128,
}

pub struct autoBet {
    pub prediction: Prediction,
    pub initial_amount: Uint128,
    pub increase_percentage_upon_win: Option<u8>,
    pub decrese_percentage_upon_loose: Option<u8>,
    pub stoploss: Option<Uint128>,
    pub target: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Prediction {
    Head,
    Tail,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum FlipResult {
    Win,
    Lose,
}
impl std::fmt::Display for FlipResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FlipResult::Win => write!(f, "Heads"),
            FlipResult::Lose => write!(f, "Tails"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct Game {
    pub owner: Addr,
    pub seed: String,
    pub last_game_time: u64,
    pub gameId: u128,
    pub bet: bet,
}

pub const GAME: Item<Game> = Item::new("game");
pub const GAMEID: Map<Addr, Vec<u128>> = Map::new("gameID");

pub const USERBET: Map<Addr, bet> = Map::new("user_bet");
