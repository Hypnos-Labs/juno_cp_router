use cosmwasm_schema::{cw_serde};

#[cw_serde]
#[cfg_attr(test, derive(Default))]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Route,
    RouteWithSender
}
