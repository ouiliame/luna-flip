use std::cmp::min;

use cosmwasm_std::{
    generic_err, log, to_binary, Api, Binary, CanonicalAddr, Env, Extern, HandleResponse,
    InitResponse, Querier, StdResult, Storage,
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg, WinnerResponse};
use crate::state::{
    get_config, get_count, get_players, get_prevote, get_status, get_vote, get_winner, set_config,
    set_count, set_players, set_prevote, set_status, set_vote, set_winner, Config, Status,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
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
        set_players(&mut deps.storage, &players)?;
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
            for player in &players {
                x.push_str(get_vote(&deps.storage, &player)?.as_str());
            }
            let mut hasher = DefaultHasher::default();
            hasher.write(x.as_bytes());
            let result = hasher.finish() % num_players as u64;
            let winner = &players[result as usize];
            set_winner(&mut deps.storage, &winner)?;
            set_status(&mut deps.storage, &Status::Done)?;
            let res = HandleResponse {
                messages: vec![],
                log: vec![log("winner", deps.api.human_address(winner)?.as_str())],
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

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Winner {} => {
            let winner = get_winner(&deps.storage)?;
            Ok(to_binary(&WinnerResponse {
                winner: deps.api.human_address(&winner)?,
            })?)
        }
        _ => Ok(to_binary(&0)?),
    }
}
