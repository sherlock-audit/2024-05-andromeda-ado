use andromeda_std::{
    amp::recipient::Recipient, andr_exec, andr_instantiate, andr_query, error::ContractError,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{ensure, Decimal, ReplyOn};
use cw_utils::Expiration;

#[cw_serde]
pub struct AddressPercent {
    pub recipient: Recipient,
    pub percent: Decimal,
}

impl AddressPercent {
    pub fn new(recipient: Recipient, percent: Decimal) -> Self {
        Self { recipient, percent }
    }
}

#[cw_serde]
/// A config struct for a `Splitter` contract.
pub struct Splitter {
    /// The vector of recipients for the contract. Anytime a `Send` execute message is sent the amount sent will be divided amongst these recipients depending on their assigned percentage.
    pub recipients: Vec<AddressPercent>,
    /// Whether or not the contract is currently locked. This restricts updating any config related fields.
    pub lock: Expiration,
}

#[andr_instantiate]
#[cw_serde]
pub struct InstantiateMsg {
    /// The vector of recipients for the contract. Anytime a `Send` execute message is
    /// sent the amount sent will be divided amongst these recipients depending on their assigned percentage.
    pub recipients: Vec<AddressPercent>,
    pub lock_time: Option<u64>,
}

impl InstantiateMsg {
    pub fn validate(&self) -> Result<bool, ContractError> {
        validate_recipient_list(self.recipients.clone())?;
        Ok(true)
    }
}

#[cw_serde]
pub struct ReplyGasExit {
    pub reply_on: Option<ReplyOn>,
    pub gas_limit: Option<u64>,
    pub exit_at_error: Option<bool>,
}

#[andr_exec]
#[cw_serde]
pub enum ExecuteMsg {
    /// Update the recipients list. Only executable by the contract owner when the contract is not locked.
    UpdateRecipients { recipients: Vec<AddressPercent> },
    /// Used to lock/unlock the contract allowing the config to be updated.
    UpdateLock { lock_time: u64 },
    /// Divides any attached funds to the message amongst the recipients list.
    Send {},
}

#[cw_serde]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

#[andr_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// The current config of the Splitter contract
    #[returns(GetSplitterConfigResponse)]
    GetSplitterConfig {},
}

#[cw_serde]
pub struct GetSplitterConfigResponse {
    pub config: Splitter,
}

/// Ensures that a given list of recipients for a `splitter` contract is valid:
///
/// * Must include at least one recipient
/// * The combined percentage of the recipients must not exceed 100
pub fn validate_recipient_list(recipients: Vec<AddressPercent>) -> Result<bool, ContractError> {
    ensure!(
        !recipients.is_empty(),
        ContractError::EmptyRecipientsList {}
    );

    let mut percent_sum: Decimal = Decimal::zero();
    for rec in recipients {
        // += operation is not supported for decimal.
        percent_sum += rec.percent;
    }

    ensure!(
        percent_sum <= Decimal::one(),
        ContractError::AmountExceededHundredPrecent {}
    );

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_recipient_list() {
        let empty_recipients = vec![];
        let res = validate_recipient_list(empty_recipients).unwrap_err();
        assert_eq!(res, ContractError::EmptyRecipientsList {});

        let inadequate_recipients = vec![AddressPercent {
            recipient: Recipient::from_string(String::from("Some Address")),
            percent: Decimal::percent(150),
        }];
        let res = validate_recipient_list(inadequate_recipients).unwrap_err();
        assert_eq!(res, ContractError::AmountExceededHundredPrecent {});

        let valid_recipients = vec![
            AddressPercent {
                recipient: Recipient::from_string(String::from("Some Address")),
                percent: Decimal::percent(50),
            },
            AddressPercent {
                recipient: Recipient::from_string(String::from("Some Address")),
                percent: Decimal::percent(50),
            },
        ];

        let res = validate_recipient_list(valid_recipients).unwrap();
        assert!(res);
    }
}
