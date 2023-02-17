
#[cfg(not(feature = "library"))]
pub use cosmwasm_std::{
    coin, from_binary, has_coins, to_binary, entry_point, Coin, StdError, Addr, Binary, CosmosMsg, Deps, DepsMut, Empty,
    Env, MessageInfo, Order, QueryRequest, Reply, ReplyOn, Response, StdResult, SubMsg,
    Uint128, WasmMsg, WasmQuery,
};
use cosmos_sdk_proto::cosmos::distribution::v1beta1::MsgFundCommunityPool;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin as SdkCoin;

pub use cw2::set_contract_version;
pub use crate::error::*;
pub use crate::msg::*;

const CONTRACT_NAME: &str = "crates.io:cp_router";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Instantiate
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
    )
}

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Execute
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Route => {
            route(env, info)
        }
        ExecuteMsg::RouteWithSender => {
            route_with_sender(info)
        }
    }
}

pub fn route(
    env: Env,
    info: MessageInfo
) -> Result<Response, ContractError> {

    if info.funds.is_empty() {
        return Err(ContractError::GenericError("Msg: Route | Error: funds.is_empty".to_string()));
    }

    let mut msgs: Vec<CosmosMsg> = vec![];

    let mut attr: Vec<String> = vec![];

    for coin in info.funds {
        attr.push(format!("{}: {}", coin.denom.clone(), coin.amount.to_string()));
        let msg = coin.get_cp_msg(&env.contract.address)?;
        msgs.push(msg);
    }

    Ok(Response::new()
        .add_attribute("funds sent ||| ", attr.join(" | "))
        .add_attribute("from address: ", env.contract.address.to_string())
        .add_messages(msgs)
    )
}

pub fn route_with_sender(
    info: MessageInfo,
) -> Result<Response, ContractError> {
    if info.funds.is_empty() {
        return Err(ContractError::GenericError("Msg: RouteWithSender | Error: funds.is_empty".to_string()));
    }

    let mut msgs: Vec<CosmosMsg> = vec![];

    let mut attr: Vec<String> = vec![];

    for coin in info.funds {
        attr.push(format!("{}: {}", coin.denom.clone(), coin.amount.to_string()));
        let msg = coin.get_cp_msg(&info.sender)?;
        msgs.push(msg);
    }

    Ok(Response::new()
        .add_attribute("funds sent ||| ", attr.join(" | "))
        .add_attribute("from address: ", info.sender.to_string())
        .add_messages(msgs)
    )
}


pub trait GetComPoolMsg {
    fn get_cp_msg(&self, depositor: &Addr) -> Result<CosmosMsg, ContractError>;
}

impl GetComPoolMsg for Coin {
    fn get_cp_msg(&self, depositor: &Addr) -> Result<CosmosMsg, ContractError> {
        Ok(proto_encode(
            MsgFundCommunityPool {
                amount: vec![SdkCoin {
                    denom: self.denom.to_string(),
                    amount: self.amount.to_string(),
                }],
                depositor: depositor.to_string(),
            },
            "/cosmos.distribution.v1beta1.MsgFundCommunityPool".to_string(),
        )?)
    }
}

pub fn proto_encode<M: prost::Message>(msg: M, type_url: String) -> StdResult<CosmosMsg> {
    let mut bytes = Vec::new();
    prost::Message::encode(&msg, &mut bytes)
        .map_err(|_e| StdError::generic_err("Message encoding must be infallible"))?;
    Ok(cosmwasm_std::CosmosMsg::<cosmwasm_std::Empty>::Stargate {
        type_url,
        value: cosmwasm_std::Binary(bytes),
    })
}
