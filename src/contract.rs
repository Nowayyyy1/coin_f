use crate::random::{self, seed};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128, WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{bet, FlipResult, Game, Prediction, GAME, GAMEID, USERBET};
use cw20::Cw20ExecuteMsg;

use random::pcg64_from_game_seed;
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

    let state = Game {
        owner: _info.sender,
        seed: seed::init(&String::from("seed"), _env.block.height, _env),
        last_game_time: 0,
        gameId: 0,
        bet: bet {
            prediction: Prediction::Head,
            amount: Uint128::new(10),
        },
    };

    GAME.save(_deps.storage, &state);

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
        ExecuteMsg::FlipCoin {} => flip_coin(_deps, _env, _info),
        ExecuteMsg::TakeBet { bet } => take_bet(_deps, _env, _info, bet),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}
pub fn take_bet(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    order: bet,
) -> Result<Response, ContractError> {
    let mut game: Game = GAME.load(_deps.storage)?;

    game.bet = order.clone();
    GAME.save(_deps.storage, &game);
    USERBET.save(_deps.storage, _info.sender.clone(), &order);

    Ok(Response::new()
        .add_attribute("bet", "created")
        .add_attribute("for user", _info.sender.to_string()))
}

pub fn flip_coin(_deps: DepsMut, _env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let user = _info.sender.clone();
    let order = USERBET.load(_deps.storage, user.clone())?;
    let coin = get_coin(_deps.as_ref(), _env, _info)?;

    let result: FlipResult = match coin {
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

fn get_coin(_deps: Deps, _env: Env, _info: MessageInfo) -> Result<Prediction, ContractError> {
    let hash = get_random_number(_deps, _env, _info)?;
    let mut rng = pcg64_from_game_seed(&hash.clone()).unwrap();
    let random_number = rng.next_u64();
    println!("rng generated : {:?}", random_number);

    if random_number % 2 == 0 {
        return Ok(Prediction::Head);
    } else {
        return Ok(Prediction::Tail);
    }
}

fn get_random_number(_deps: Deps, _env: Env, _info: MessageInfo) -> Result<String, ContractError> {
    let mut game: Game = GAME.load(_deps.storage)?;
    game.owner = _info.sender.clone();
    let owner = _info.sender;
    game.gameId = game.gameId + 1;

    let hash = random::seed::update(&game, &owner, game.gameId, _env.block.height);
    game.seed = hash.clone();

    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{
        coin, coins, Addr, BankMsg, BlockInfo, ContractInfo, StakingMsg, SubMsg, Timestamp,
        TransactionInfo, WasmMsg,
    };
    use sha2::{Digest, Sha256};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let res = instantiate(deps.as_mut(), mock_env(), mock_info("advock", &[]), msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]

    fn execute_play_game() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let _res = instantiate(deps.as_mut(), mock_env(), mock_info("advock", &[]), msg).unwrap();
        pub const MOCK_CONTRACT_ADDR: &str = "cosmos2contract";

        let env1 = Env {
            block: BlockInfo {
                height: 12_315,
                time: Timestamp::from_nanos(1_571_797_419_879_305_533),
                chain_id: "cosmos-testnet-14002".to_string(),
            },
            transaction: Some(TransactionInfo { index: 0 }),
            contract: ContractInfo {
                address: Addr::unchecked(MOCK_CONTRACT_ADDR),
            },
        };
        println!("tx1");

        let order = bet {
            prediction: Prediction::Head,
            amount: Uint128::new(100),
        };

        let msg = ExecuteMsg::TakeBet { bet: order };

        let _res = execute(deps.as_mut(), env1, mock_info("advcok", &[]), msg).unwrap();

        println!("res: {:?}", _res);

        println!("tx2");

        let env2 = Env {
            block: BlockInfo {
                height: 12_316,
                time: Timestamp::from_nanos(1_571_797_419_879_305_533),
                chain_id: "cosmos-testnet-14002".to_string(),
            },
            transaction: Some(TransactionInfo { index: 0 }),
            contract: ContractInfo {
                address: Addr::unchecked(MOCK_CONTRACT_ADDR),
            },
        };

        let msg2 = ExecuteMsg::FlipCoin {};

        let _res2 = execute(deps.as_mut(), env2, mock_info("advcok", &[]), msg2).unwrap();

        print!("res2: {:?}", _res2);

        // println!("game 2");

        // let env = Env {
        //     block: BlockInfo {
        //         height: 12_545,
        //         time: Timestamp::from_nanos(1_571_797_419_879_305_533),
        //         chain_id: "cosmos-testnet-14002".to_string(),
        //     },
        //     transaction: Some(TransactionInfo { index: 1 }),
        //     contract: ContractInfo {
        //         address: Addr::unchecked(MOCK_CONTRACT_ADDR),
        //     },
        // };
        // let msg = ExecuteMsg::FlipCoin {
        //     prediction: Prediction::Tail,
        //     amount: Uint128::new(1000),
        // };

        // let _res = execute(deps.as_mut(), env, mock_info("advock2", &[]), msg).unwrap();

        // println!("res: {:?}", _res);
    }
}
