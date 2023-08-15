#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{bet, FlipResult, Prediction, GAMEID};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:coin-flip";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

const CONTRACT_NAME: &str = "crates.io:coin-flip";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::FlipCoin { prediction, amount } => {
            let order = bet { prediction, amount };
            flip_coin(_deps, _env, _info, order)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

fn get_random_number(_deps: Deps, _env: Env) -> Result<u128, ContractError> {
    unimplemented!()
}

fn get_coin(_deps: Deps, _env: Env) -> Result<Prediction, ContractError> {
    let random_number = get_random_number(_deps, _env)?;
    if random_number % 2 == 0 {
        return Ok(Prediction::Head);
    } else {
        return Ok(Prediction::Tail);
    }
}

pub fn flip_coin(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    order: bet,
) -> Result<Response, ContractError> {
    let user = _info.sender;
    let coin = get_coin(_deps.as_ref(), _env)?;

    let result = match coin {
        Prediction::Head => {
            if order.prediction == Prediction::Head {
                FlipResult::Win
            } else {
                FlipResult::Lose
            }
        }
        Prediction::Tail => {
            if order.prediction == Prediction::Tail {
                FlipResult::Win
            } else {
                FlipResult::Lose
            }
        }
    };

    Ok(Response::new()
        .add_attribute("result", result.to_string())
        .add_attribute("user", user))
}

#[cfg(test)]
mod tests {}
