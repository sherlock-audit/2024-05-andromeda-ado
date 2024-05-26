// The mars lockdrop contract was used as a base for this.
// https://github.com/mars-protocol/mars-periphery/tree/main/contracts/lockdrop

use andromeda_fungible_tokens::lockdrop::{
    ConfigResponse, Cw20HookMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, StateResponse,
    UserInfoResponse,
};
use andromeda_std::{
    ado_base::{hooks::AndromedaHook, InstantiateMsg as BaseInstantiateMsg},
    ado_contract::ADOContract,
    common::expiration::MILLISECONDS_TO_NANOSECONDS_RATIO,
    common::{context::ExecuteContext, encode_binary},
    error::{from_semver, ContractError},
};
use cosmwasm_std::{ensure, from_json, Binary, Deps, DepsMut, Env, MessageInfo, Response, Uint128};
use cosmwasm_std::{entry_point, Decimal};
use cw_asset::Asset;

use crate::state::{Config, State, CONFIG, STATE, USER_INFO};
use cw2::{get_contract_version, set_contract_version};
use cw20::Cw20ReceiveMsg;

use cw_utils::nonpayable;
use semver::Version;

// version info for migration info
const CONTRACT_NAME: &str = "andromeda-lockdrop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

//----------------------------------------------------------------------------------------
// Entry Points
//----------------------------------------------------------------------------------------

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // CHECK :: init_timestamp needs to be valid
    ensure!(
        msg.init_timestamp >= env.block.time.seconds(),
        ContractError::StartTimeInThePast {
            current_time: env.block.time.nanos() / MILLISECONDS_TO_NANOSECONDS_RATIO,
            current_block: env.block.height,
        }
    );

    // CHECK :: deposit_window,withdrawal_window need to be valid (withdrawal_window < deposit_window)
    ensure!(
        msg.deposit_window > 0
            && msg.withdrawal_window > 0
            && msg.withdrawal_window < msg.deposit_window,
        ContractError::InvalidWindow {}
    );

    let config = Config {
        // bootstrap_contract_address: msg.bootstrap_contract,
        init_timestamp: msg.init_timestamp,
        deposit_window: msg.deposit_window,
        withdrawal_window: msg.withdrawal_window,
        lockdrop_incentives: Uint128::zero(),
        incentive_token: msg.incentive_token,
        native_denom: msg.native_denom,
    };

    CONFIG.save(deps.storage, &config)?;
    STATE.save(deps.storage, &State::default())?;

    let inst_resp = ADOContract::default().instantiate(
        deps.storage,
        env,
        deps.api,
        info.clone(),
        BaseInstantiateMsg {
            ado_type: "lockdrop".to_string(),
            ado_version: CONTRACT_VERSION.to_string(),
            operators: None,
            kernel_address: msg.kernel_address,
            owner: msg.owner,
        },
    )?;
    let mod_resp =
        ADOContract::default().register_modules(info.sender.as_str(), deps.storage, msg.modules)?;

    Ok(inst_resp
        .add_attributes(mod_resp.attributes)
        .add_submessages(mod_resp.messages))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let ctx = ExecuteContext::new(deps, info, env);

    match msg {
        ExecuteMsg::AMPReceive(pkt) => {
            ADOContract::default().execute_amp_receive(ctx, pkt, handle_execute)
        }
        _ => handle_execute(ctx, msg),
    }
}

pub fn handle_execute(ctx: ExecuteContext, msg: ExecuteMsg) -> Result<Response, ContractError> {
    let contract = ADOContract::default();

    if !matches!(msg, ExecuteMsg::UpdateAppContract { .. })
        && !matches!(msg, ExecuteMsg::UpdateOwner { .. })
    {
        contract.module_hook::<Response>(
            &ctx.deps.as_ref(),
            AndromedaHook::OnExecute {
                sender: ctx.info.sender.to_string(),
                payload: encode_binary(&msg)?,
            },
        )?;
    }

    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(ctx, msg),
        ExecuteMsg::DepositNative {} => execute_deposit_native(ctx),
        ExecuteMsg::WithdrawNative { amount } => execute_withdraw_native(ctx, amount),
        ExecuteMsg::EnableClaims {} => execute_enable_claims(ctx),
        ExecuteMsg::ClaimRewards {} => execute_claim_rewards(ctx),
        ExecuteMsg::WithdrawProceeds { recipient } => execute_withdraw_proceeds(ctx, recipient),

        _ => handle_execute(ctx, msg),
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

pub fn receive_cw20(
    ctx: ExecuteContext,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    // CHECK :: Tokens sent > 0
    ensure!(
        !cw20_msg.amount.is_zero(),
        ContractError::InvalidFunds {
            msg: "Number of tokens should be > 0".to_string(),
        }
    );

    match from_json(&cw20_msg.msg)? {
        Cw20HookMsg::IncreaseIncentives {} => execute_increase_incentives(ctx, cw20_msg.amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Config {} => encode_binary(&query_config(deps)?),
        QueryMsg::State {} => encode_binary(&query_state(deps)?),
        QueryMsg::UserInfo { address } => encode_binary(&query_user_info(deps, env, address)?),
        QueryMsg::WithdrawalPercentAllowed { timestamp } => {
            encode_binary(&query_max_withdrawable_percent(deps, env, timestamp)?)
        }
        _ => ADOContract::default().query(deps, env, msg),
    }
}

//----------------------------------------------------------------------------------------
// Execute Functions
//----------------------------------------------------------------------------------------

/// @dev Facilitates increasing token incentives that are to be distributed as Lockdrop participation reward
/// @params amount : Number of tokens which are to be added to current incentives
pub fn execute_increase_incentives(
    ctx: ExecuteContext,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let ExecuteContext {
        deps, env, info, ..
    } = ctx;
    let mut config = CONFIG.load(deps.storage)?;

    ensure!(
        info.sender == config.incentive_token,
        ContractError::InvalidFunds {
            msg: "Only incentive tokens are valid".to_string(),
        }
    );

    ensure!(
        is_withdraw_open(env.block.time.seconds(), &config),
        ContractError::TokenAlreadyBeingDistributed {}
    );

    config.lockdrop_incentives += amount;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "incentives_increased")
        .add_attribute("amount", amount))
}

/// @dev Facilitates NATIVE deposits.
pub fn execute_deposit_native(ctx: ExecuteContext) -> Result<Response, ContractError> {
    let ExecuteContext {
        deps, env, info, ..
    } = ctx;
    let config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;

    let depositor_address = info.sender;

    // CHECK :: Lockdrop deposit window open
    ensure!(
        is_deposit_open(env.block.time.seconds(), &config),
        ContractError::DepositWindowClosed {}
    );

    // Check if multiple native coins sent by the user
    ensure!(
        info.funds.len() == 1,
        ContractError::InvalidFunds {
            msg: "Must deposit a single fund".to_string(),
        }
    );

    let native_token = info.funds.first().unwrap();
    ensure!(
        native_token.denom == config.native_denom,
        ContractError::InvalidFunds {
            msg: format!("Only {} accepted", config.native_denom),
        }
    );

    // CHECK ::: Amount needs to be valid
    ensure!(
        !native_token.amount.is_zero(),
        ContractError::InvalidFunds {
            msg: "Amount must be greater than 0".to_string(),
        }
    );

    // USER INFO :: RETRIEVE --> UPDATE
    let mut user_info = USER_INFO
        .may_load(deps.storage, &depositor_address)?
        .unwrap_or_default();

    user_info.total_native_locked += native_token.amount;

    // STATE :: UPDATE --> SAVE
    state.total_native_locked += native_token.amount;

    STATE.save(deps.storage, &state)?;
    USER_INFO.save(deps.storage, &depositor_address, &user_info)?;

    Ok(Response::new()
        .add_attribute("action", "lock_native")
        .add_attribute("user", depositor_address)
        .add_attribute("ust_deposited", native_token.amount))
}

/// @dev Facilitates NATIVE withdrawal from an existing Lockup position. Can only be called when deposit / withdrawal window is open
/// @param withdraw_amount : NATIVE amount to be withdrawn
pub fn execute_withdraw_native(
    ctx: ExecuteContext,
    withdraw_amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    let ExecuteContext {
        deps, env, info, ..
    } = ctx;
    let config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;

    let mut user_info = USER_INFO.load(deps.storage, &info.sender)?;

    // USER ADDRESS AND LOCKUP DETAILS
    let withdrawer_address = info.sender;

    // CHECK :: Lockdrop withdrawal window open
    ensure!(
        is_withdraw_open(env.block.time.seconds(), &config),
        ContractError::InvalidWithdrawal {
            msg: Some("Withdrawals not available".to_string()),
        }
    );

    // Check :: Amount should be within the allowed withdrawal limit bounds
    let max_withdrawal_percent = allowed_withdrawal_percent(env.block.time.seconds(), &config);
    let max_withdrawal_allowed = user_info.total_native_locked * max_withdrawal_percent;
    let withdraw_amount = withdraw_amount.unwrap_or(max_withdrawal_allowed);
    ensure!(
        withdraw_amount <= max_withdrawal_allowed,
        ContractError::InvalidWithdrawal {
            msg: Some(format!(
                "Amount exceeds max allowed withdrawal limit of {max_withdrawal_allowed}"
            )),
        }
    );

    // Update withdrawal flag after the deposit window
    if env.block.time.seconds() > config.init_timestamp + config.deposit_window {
        // CHECK :: Max 1 withdrawal allowed
        ensure!(
            !user_info.withdrawal_flag,
            ContractError::InvalidWithdrawal {
                msg: Some("Max 1 withdrawal allowed".to_string()),
            }
        );

        user_info.withdrawal_flag = true;
    }

    user_info.total_native_locked -= withdraw_amount;

    USER_INFO.save(deps.storage, &withdrawer_address, &user_info)?;

    // STATE :: UPDATE --> SAVE
    state.total_native_locked -= withdraw_amount;
    STATE.save(deps.storage, &state)?;

    // COSMOS_MSG ::TRANSFER WITHDRAWN native token
    let native_token = Asset::native(config.native_denom, withdraw_amount);
    let withdraw_msg = native_token.transfer_msg(withdrawer_address.clone())?;

    Ok(Response::new()
        .add_message(withdraw_msg)
        .add_attribute("action", "withdraw_native")
        .add_attribute("user", withdrawer_address)
        .add_attribute("amount", withdraw_amount))
}

/// Function callable only by Bootstrap contract (if it is specified) to enable TOKEN Claims by users.
/// Called along-with Bootstrap contract's LP Pool provide liquidity tx. If it is not
/// specified then anyone can execute this when the phase has ended.
pub fn execute_enable_claims(ctx: ExecuteContext) -> Result<Response, ContractError> {
    let ExecuteContext {
        deps, env, info, ..
    } = ctx;
    nonpayable(&info)?;

    // let contract = ADOContract::default();
    let config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;

    // // If bootstrap is specified then only it can enable claims.
    // if let Some(bootstrap_contract_address) = &config.bootstrap_contract_address {
    //     let app_contract = contract.get_app_contract(deps.storage)?;
    //     let bootstrap_contract_address =
    //         bootstrap_contract_address.get_address(deps.api, &deps.querier, app_contract)?;

    //     // CHECK :: ONLY BOOTSTRAP CONTRACT CAN CALL THIS FUNCTION
    //     ensure!(
    //         info.sender == bootstrap_contract_address,
    //         ContractError::Unauthorized {},
    //     )?;
    // }

    // CHECK :: Claims can only be enabled after the deposit / withdrawal windows are closed
    ensure!(
        !is_withdraw_open(env.block.time.seconds(), &config),
        ContractError::PhaseOngoing {}
    );

    // CHECK ::: Claims are only enabled once
    ensure!(
        !state.are_claims_allowed,
        ContractError::ClaimsAlreadyAllowed {}
    );
    state.are_claims_allowed = true;

    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("action", "enable_claims"))
}

/// @dev Function to claim Rewards from lockdrop.
pub fn execute_claim_rewards(ctx: ExecuteContext) -> Result<Response, ContractError> {
    let ExecuteContext { deps, info, .. } = ctx;
    let config = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    let user_address = info.sender;
    let mut user_info = USER_INFO
        .may_load(deps.storage, &user_address)?
        .unwrap_or_default();

    ensure!(
        !user_info.lockdrop_claimed,
        ContractError::LockdropAlreadyClaimed {}
    );
    ensure!(
        !user_info.total_native_locked.is_zero(),
        ContractError::NoLockup {}
    );
    ensure!(state.are_claims_allowed, ContractError::ClaimsNotAllowed {});

    let total_incentives = config
        .lockdrop_incentives
        .multiply_ratio(user_info.total_native_locked, state.total_native_locked);

    let amount_to_transfer = total_incentives - user_info.delegated_incentives;
    let token = Asset::cw20(
        deps.api.addr_validate(&config.incentive_token)?,
        amount_to_transfer,
    );
    let transfer_msg = token.transfer_msg(user_address.clone())?;
    user_info.lockdrop_claimed = true;

    USER_INFO.save(deps.storage, &user_address, &user_info)?;

    Ok(Response::new()
        .add_attribute("action", "claim_rewards")
        .add_attribute("amount", amount_to_transfer)
        .add_message(transfer_msg))
}

fn execute_withdraw_proceeds(
    ctx: ExecuteContext,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    let ExecuteContext {
        deps, env, info, ..
    } = ctx;
    nonpayable(&info)?;

    let recipient = recipient.unwrap_or_else(|| info.sender.to_string());
    let config = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;
    // CHECK :: Only Owner can call this function
    ensure!(
        ADOContract::default().is_contract_owner(deps.storage, info.sender.as_str())?,
        ContractError::Unauthorized {}
    );

    // CHECK :: Lockdrop withdrawal window should be closed
    let current_timestamp = env.block.time.seconds();
    ensure!(
        current_timestamp >= config.init_timestamp && !is_withdraw_open(current_timestamp, &config),
        ContractError::InvalidWithdrawal {
            msg: Some("Lockdrop withdrawals haven't concluded yet".to_string()),
        }
    );

    let native_token = Asset::native(config.native_denom, state.total_native_locked);

    let balance = native_token
        .info
        .query_balance(&deps.querier, env.contract.address)?;

    ensure!(
        balance >= state.total_native_locked,
        ContractError::InvalidWithdrawal {
            msg: Some("Already withdrew funds".to_string()),
        }
    );

    let transfer_msg = native_token.transfer_msg(recipient)?;

    Ok(Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "withdraw_proceeds")
        .add_attribute("amount", state.total_native_locked)
        .add_attribute("timestamp", env.block.time.seconds().to_string()))
}

//----------------------------------------------------------------------------------------
// Query Functions
//----------------------------------------------------------------------------------------

/// @dev Returns the contract's configuration
pub fn query_config(deps: Deps) -> Result<ConfigResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    // let contract = ADOContract::default();
    // let app_contract = contract.get_app_contract(deps.storage)?;
    // let bootstrap_contract_address = config
    //     .bootstrap_contract_address
    //     .map(|a| a.get_address(deps.api, &deps.querier, app_contract))
    //     // Flip Option<Result> to Result<Option>
    //     .map_or(Ok(None), |v| v.map(Some));

    Ok(ConfigResponse {
        // bootstrap_contract_address: bootstrap_contract_address?,
        init_timestamp: config.init_timestamp,
        deposit_window: config.deposit_window,
        withdrawal_window: config.withdrawal_window,
        lockdrop_incentives: config.lockdrop_incentives,
        incentive_token: config.incentive_token,
        native_denom: config.native_denom,
    })
}

/// @dev Returns the contract's Global State
pub fn query_state(deps: Deps) -> Result<StateResponse, ContractError> {
    let state: State = STATE.load(deps.storage)?;
    Ok(StateResponse {
        total_native_locked: state.total_native_locked,
        are_claims_allowed: state.are_claims_allowed,
    })
}

/// @dev Returns summarized details regarding the user
/// @params user_address : User address whose state is being queries
pub fn query_user_info(
    deps: Deps,
    _env: Env,
    user_address_: String,
) -> Result<UserInfoResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let user_address = deps.api.addr_validate(&user_address_)?;
    let state: State = STATE.load(deps.storage)?;
    let user_info = USER_INFO
        .may_load(deps.storage, &user_address)?
        .unwrap_or_default();

    let total_incentives = config
        .lockdrop_incentives
        .multiply_ratio(user_info.total_native_locked, state.total_native_locked);

    Ok(UserInfoResponse {
        total_native_locked: user_info.total_native_locked,
        total_incentives,
        is_lockdrop_claimed: user_info.lockdrop_claimed,
        withdrawal_flag: user_info.withdrawal_flag,
    })
}

/// @dev Returns max withdrawable % for a position
pub fn query_max_withdrawable_percent(
    deps: Deps,
    env: Env,
    timestamp: Option<u64>,
) -> Result<Decimal, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    Ok(match timestamp {
        Some(timestamp) => allowed_withdrawal_percent(timestamp, &config),
        None => allowed_withdrawal_percent(env.block.time.seconds(), &config),
    })
}

//----------------------------------------------------------------------------------------
// HELPERS
//----------------------------------------------------------------------------------------

/// @dev Returns true if deposits are allowed
fn is_deposit_open(current_timestamp: u64, config: &Config) -> bool {
    let deposits_opened_till = config.init_timestamp + config.deposit_window;
    (current_timestamp >= config.init_timestamp) && (deposits_opened_till >= current_timestamp)
}

/// @dev Returns true if withdrawals are allowed
fn is_withdraw_open(current_timestamp: u64, config: &Config) -> bool {
    let withdrawals_opened_till =
        config.init_timestamp + config.deposit_window + config.withdrawal_window;
    (current_timestamp >= config.init_timestamp) && (withdrawals_opened_till >= current_timestamp)
}

/// @dev Helper function to calculate maximum % of NATIVE deposited that can be withdrawn
/// @params current_timestamp : Current block timestamp
/// @params config : Contract configuration
pub fn allowed_withdrawal_percent(current_timestamp: u64, config: &Config) -> Decimal {
    let withdrawal_cutoff_init_point = config.init_timestamp + config.deposit_window;

    // Deposit window :: 100% withdrawals allowed
    if current_timestamp < withdrawal_cutoff_init_point {
        return Decimal::from_ratio(100u32, 100u32);
    }

    let withdrawal_cutoff_second_point =
        withdrawal_cutoff_init_point + (config.withdrawal_window / 2u64);
    // Deposit window closed, 1st half of withdrawal window :: 50% withdrawals allowed
    if current_timestamp <= withdrawal_cutoff_second_point {
        return Decimal::from_ratio(50u32, 100u32);
    }

    // max withdrawal allowed decreasing linearly from 50% to 0% vs time elapsed
    let withdrawal_cutoff_final = withdrawal_cutoff_init_point + config.withdrawal_window;
    //  Deposit window closed, 2nd half of withdrawal window :: max withdrawal allowed decreases linearly from 50% to 0% vs time elapsed
    if current_timestamp < withdrawal_cutoff_final {
        let time_left = withdrawal_cutoff_final - current_timestamp;
        Decimal::from_ratio(
            50u64 * time_left,
            100u64 * (withdrawal_cutoff_final - withdrawal_cutoff_second_point),
        )
    }
    // Withdrawals not allowed
    else {
        Decimal::from_ratio(0u32, 100u32)
    }
}
