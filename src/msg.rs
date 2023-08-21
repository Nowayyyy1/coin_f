use crate::state::{bet, Prediction};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    FlipCoin {},

    TakeBet { bet: bet },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
