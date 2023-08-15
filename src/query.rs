use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{bet, Prediction, GAMEID};

pub fn query_gameId(deps: Deps, user: Addr) -> StdResult<Vec<u128>> {
    let res = GAMEID.may_load(deps.storage, user.clone())?;

    match res {
        Some(val) => Ok(val),
        None => Err(StdError::NotFound {
            kind: format!("Unable to load orders with taskID: {}", user),
        }),
    }
}
