use andromeda_std::{andr_exec, andr_instantiate, andr_query};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[andr_instantiate]
#[cw_serde]
pub struct InstantiateMsg {
    pub is_inclusive: bool,
}

#[andr_exec]
#[cw_serde]
pub enum ExecuteMsg {
    /// Add an address to the address list
    AddAddress { address: String },
    /// Remove an address from the address list
    RemoveAddress { address: String },
    /// Add multiple addresses to the address list
    AddAddresses { addresses: Vec<String> },
}

#[cw_serde]
pub struct MigrateMsg {}

#[andr_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Query if address is included
    #[returns(IncludesAddressResponse)]
    IncludesAddress { address: String },
    #[returns(IsInclusiveResponse)]
    IsInclusive {},
}
#[cw_serde]
pub struct IsInclusiveResponse {
    pub is_inclusive_response: bool,
}

#[cw_serde]
pub struct IncludesAddressResponse {
    /// Whether the address is included in the address list
    pub included: bool,
}
