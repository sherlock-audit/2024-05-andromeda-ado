use andromeda_std::ado_contract::ADOContract;

use andromeda_std::os::vfs::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use andromeda_std::{
    ado_base::InstantiateMsg as BaseInstantiateMsg, common::encode_binary, error::ContractError,
};
use cosmwasm_std::{
    ensure, entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
};
use cw2::{get_contract_version, set_contract_version};
use semver::Version;

use crate::{execute, query};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:andromeda-vfs";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    ADOContract::default().instantiate(
        deps.storage,
        env,
        deps.api,
        info,
        BaseInstantiateMsg {
            ado_type: "vfs".to_string(),
            ado_version: CONTRACT_VERSION.to_string(),
            operators: None,
            kernel_address: msg.kernel_address,
            owner: msg.owner,
        },
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.result.is_err() {
        return Err(ContractError::Std(StdError::generic_err(
            msg.result.unwrap_err(),
        )));
    }

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let execute_env = execute::ExecuteEnv { deps, env, info };

    match msg {
        ExecuteMsg::AddPath {
            name,
            address,
            parent_address,
        } => execute::add_path(execute_env, name, address, parent_address),
        ExecuteMsg::AddSymlink {
            name,
            symlink,
            parent_address,
        } => execute::add_symlink(execute_env, name, symlink, parent_address),
        ExecuteMsg::RegisterUser { username, address } => {
            execute::register_user(execute_env, username, address)
        }
        ExecuteMsg::AddParentPath {
            name,
            parent_address,
        } => execute::add_parent_path(execute_env, name, parent_address),
        ExecuteMsg::RegisterLibrary {
            lib_name,
            lib_address,
        } => execute::register_library(execute_env, lib_name, lib_address),
        ExecuteMsg::RegisterUserCrossChain { chain, address } => {
            execute::register_user_cross_chain(execute_env, chain, address)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    // New version
    let version: Version = CONTRACT_VERSION.parse().map_err(from_semver)?;

    // Old version
    let stored = get_contract_version(deps.storage)?;
    let storage_version: Version = stored.version.parse().map_err(from_semver)?;

    let contract = ADOContract::default();

    ensure!(
        stored.contract == CONTRACT_NAME,
        ContractError::CannotMigrate {
            previous_contract: stored.contract,
        }
    );

    // New version has to be newer/greater than the old version
    ensure!(
        storage_version < version,
        ContractError::CannotMigrate {
            previous_contract: stored.version,
        }
    );

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Update the ADOContract's version
    contract.execute_update_version(deps)?;

    Ok(Response::default())
}

fn from_semver(err: semver::Error) -> StdError {
    StdError::generic_err(format!("Semver: {err}"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::ResolvePath { path } => encode_binary(&query::resolve_path(deps, path)?),
        QueryMsg::SubDir { path } => encode_binary(&query::subdir(deps, path)?),
        QueryMsg::Paths { addr } => encode_binary(&query::paths(deps, addr)?),
        QueryMsg::GetUsername { address } => encode_binary(&query::get_username(deps, address)?),
        QueryMsg::GetLibrary { address } => encode_binary(&query::get_library_name(deps, address)?),
        QueryMsg::ResolveSymlink { path } => encode_binary(&query::get_symlink(deps, path)?),
    }
}
