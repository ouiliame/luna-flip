use std::cmp::min;

use cosmwasm_std::{
    generic_err, to_binary, to_vec, unauthorized, Api, Binary, CanonicalAddr, Coin, Env, Extern,
    HandleResponse, InitResponse, Querier, QueryRequest, StdResult, Storage, Uint128,
};

use terra_bindings::{create_swap_msg, TerraMsgWrapper, TerraQuerier, TerraQueryWrapper};

use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{
    get_config, get_count, get_players, get_prevote, get_status, get_vote, set_config, set_count,
    set_players, set_prevote, set_status, set_vote, Config, Status,
};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    set_config(
        &mut deps.storage,
        &Config {
            price: msg.price,
            players: msg.players,
        },
    )?;
    set_count(&mut deps.storage, 0)?;
    set_status(&mut deps.storage, &Status::PrevoteStage)?;
    set_players(&mut deps.storage, &Vec::<CanonicalAddr>::new())?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Prevote { prevote } => try_prevote(deps, env, &prevote),
        HandleMsg::Vote { vote } => try_vote(deps, env, &vote),
    }
}

pub fn try_prevote<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    prevote: &String,
) -> StdResult<HandleResponse> {
    let status = get_status(&deps.storage)?;
    let config = get_config(&deps.storage)?;
    let num_players = config.players;
    let mut players = get_players(&deps.storage)?;
    let mut count = get_count(&deps.storage)?;
    if status == Status::PrevoteStage {
        set_prevote(&mut deps.storage, &env.message.sender, prevote)?;
        players.push(env.message.sender);
        count += 1;
        if count == num_players {
            set_count(&mut deps.storage, 0)?;
            set_status(&mut deps.storage, &Status::VoteStage)?;
        } else {
            set_count(&mut deps.storage, count)?;
        }
        let res = HandleResponse {
            messages: vec![],
            data: None,
            log: vec![],
        };
        Ok(res)
    } else {
        Err(generic_err("Not at prevote stage."))
    }
}

pub fn try_vote<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    vote: &String,
) -> StdResult<HandleResponse> {
    let status = get_status(&deps.storage)?;
    let config = get_config(&deps.storage)?;
    let num_players = config.players;
    let players = get_players(&deps.storage)?;
    let mut count = get_count(&deps.storage)?;
    if status == Status::VoteStage {
        set_vote(&mut deps.storage, &env.message.sender, vote)?;
        count += 1;
        if count == num_players {
            let mut x = String::new();
            for player in players {
                x.push_str(get_vote(&deps.storage, &player)?.as_str());
            }
            set_status(&mut deps.storage, &Status::Done)?;
            let res = HandleResponse {
                messages: vec![],
                log: vec![],
                data: None,
            };
            Ok(res)
        } else {
            set_count(&mut deps.storage, count)?;
            let res = HandleResponse {
                messages: vec![],
                log: vec![],
                data: None,
            };
            Ok(res)
        }
    } else {
        Err(generic_err("Not at vote stage."))
    }
}
