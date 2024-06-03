
# Andromeda ADO  contest details

- Join [Sherlock Discord](https://discord.gg/MABEWyASkp)
- Submit findings using the issue page in your private contest repo (label issues as med or high)
- [Read for more details](https://docs.sherlock.xyz/audits/watsons)

# Q&A

### Q: On what chains are the smart contracts going to be deployed?
CosmWasm enabled Cosmos Chains
___

### Q: If you are integrating tokens, are you allowing only whitelisted tokens to work with the codebase or any complying with the standard? Are they assumed to have certain properties, e.g. be non-reentrant? Are there any types of <a href="https://github.com/d-xo/weird-erc20" target="_blank" rel="noopener noreferrer">weird tokens</a> you want to integrate?
Any compatible CW20 Smart contract and any Cosmos chain native token.

We can assume the tokens are non-reentrant.

___

### Q: Are the admins of the protocols your contracts integrate with (if any) TRUSTED or RESTRICTED? If these integrations are trusted, should auditors also assume they are always responsive, for example, are oracles trusted to provide non-stale information, or VRF providers to respond within a designated timeframe?
Adminship of the protocol (and relevant OS contracts) is run by the team currently (likely to shift to a DAO in future) and should be TRUSTED
___

### Q: Are there any protocol roles? Please list them and provide whether they are TRUSTED or RESTRICTED, or provide a more comprehensive description of what a role can and can't do/impact.
Contract Owner 
can whitelist/blacklist actions in the contract using the permissioning system
can alter the default staking validator for the validator staking contract
can alter operating system contract address reference
restricted to one address

Contract owner should be assumed trusted

OS contracts (kernel/vfs/adodb/economics) owner should assume to be protocol owned and TRUSTED
___

### Q: For permissioned functions, please list all checks and requirements that will be made before calling the function.
Checks for contract owner restricted functions are done by a simple address/state comparison in `ADOContract::is_contract_owner/is_contract_owner_or_operator`. The Operator role is no longer used and the latter function is just a remnant that will be removed.
___

### Q: Is the codebase expected to comply with any EIPs? Can there be/are there any deviations from the specification?
No
___

### Q: Are there any off-chain mechanisms or off-chain procedures for the protocol (keeper bots, arbitrage bots, etc.)?
No
___

### Q: Are there any hardcoded values that you intend to change before (some) deployments?
No
___

### Q: If the codebase is to be deployed on an L2, what should be the behavior of the protocol in case of sequencer issues (if applicable)? Should Sherlock assume that the Sequencer won't misbehave, including going offline?
No plans to deploy to an L2 right now
___

### Q: Should potential issues, like broken assumptions about function behavior, be reported if they could pose risks in future integrations, even if they might not be an issue in the context of the scope? If yes, can you elaborate on properties/invariants that should hold?
Yes, for the validator staking contract the amount returned for staked tokens (and claimable rewards) should match what is present in the staking module.
Permissioning functions in ADOContract should be manipulatable ONLY by the contract owner.
___

### Q: Please discuss any design choices you made.
Both contracts are fairly boilerplate, no individual decisions were made recently. Both contracts implement an `AMPReceive` handler that allows for an `origin` field similar to EVM, this may be used for permissioning but is not used for any fee calculations (for these contracts in particular).

Both contracts also implement a base structure called `ADOContract` that exposes several common entry points for the contracts.
For the vesting contract the current recipient is the owner, this would be quite likely to be changed to be a recipient address and the delegation methods would be restricted to the recipient rather than the owner.
___

### Q: Please list any known issues/acceptable risks that should not result in a valid finding.
 - Authorisation of the “origin” field in an AMP packet is done via a set of authorised contracts in the ADODB contract. This field should be assumed RESTRICTED. Any risk is acceptable unless it’s High severity
 - Any user can stake in the validator-staking contract to allow other contracts the user may create to stake on their behalf. This is an intentional design decision. Any risk is acceptable unless it is High severity.
 - The `call_action` function sends an execute message to another contract. This is determined via proxy using the `kernel_address` field. This is assumed to be the protocol’s contract and assumed TRUSTED. Any risk is acceptable unless it is High severity.
___

### Q: We will report issues where the core protocol functionality is inaccessible for at least 7 days. Would you like to override this value?
7 days is suitable
___

### Q: Please provide links to previous audits (if any).
Two previous audits can be found here, one covers a select few ADOs and the other covers the operating system smart contracts
https://github.com/andromedaprotocol/audits/tree/main/Smart-Contract-Audits
___

### Q: Please list any relevant protocol resources.
https://docs.andromedaprotocol.io/andromeda
https://www.andromedaprotocol.io/whitepaper
https://www.andromedaprotocol.io/

___

### Q: Additional audit information.
A diff from 1.0.0 to current can be found here - https://github.com/andromedaprotocol/andromeda-core/compare/development...v1.0.x

No contracts were forked for the two provided contracts

___



# Audit scope


[andromeda-core @ 676c9833f0813939c0a4f8dee60fd9feb4230e01](https://github.com/andromedaprotocol/andromeda-core/tree/676c9833f0813939c0a4f8dee60fd9feb4230e01)
- [andromeda-core/contracts/finance/andromeda-validator-staking/src/contract.rs](andromeda-core/contracts/finance/andromeda-validator-staking/src/contract.rs)
- [andromeda-core/contracts/finance/andromeda-validator-staking/src/lib.rs](andromeda-core/contracts/finance/andromeda-validator-staking/src/lib.rs)
- [andromeda-core/contracts/finance/andromeda-validator-staking/src/state.rs](andromeda-core/contracts/finance/andromeda-validator-staking/src/state.rs)
- [andromeda-core/contracts/finance/andromeda-vesting/src/contract.rs](andromeda-core/contracts/finance/andromeda-vesting/src/contract.rs)
- [andromeda-core/contracts/finance/andromeda-vesting/src/lib.rs](andromeda-core/contracts/finance/andromeda-vesting/src/lib.rs)
- [andromeda-core/contracts/finance/andromeda-vesting/src/state.rs](andromeda-core/contracts/finance/andromeda-vesting/src/state.rs)
- [andromeda-core/packages/andromeda-finance/src/lib.rs](andromeda-core/packages/andromeda-finance/src/lib.rs)
- [andromeda-core/packages/andromeda-finance/src/validator_staking.rs](andromeda-core/packages/andromeda-finance/src/validator_staking.rs)
- [andromeda-core/packages/andromeda-finance/src/vesting.rs](andromeda-core/packages/andromeda-finance/src/vesting.rs)
- [andromeda-core/packages/std/src/ado_base/ado_type.rs](andromeda-core/packages/std/src/ado_base/ado_type.rs)
- [andromeda-core/packages/std/src/ado_base/block_height.rs](andromeda-core/packages/std/src/ado_base/block_height.rs)
- [andromeda-core/packages/std/src/ado_base/hooks.rs](andromeda-core/packages/std/src/ado_base/hooks.rs)
- [andromeda-core/packages/std/src/ado_base/kernel_address.rs](andromeda-core/packages/std/src/ado_base/kernel_address.rs)
- [andromeda-core/packages/std/src/ado_base/mod.rs](andromeda-core/packages/std/src/ado_base/mod.rs)
- [andromeda-core/packages/std/src/ado_base/modules.rs](andromeda-core/packages/std/src/ado_base/modules.rs)
- [andromeda-core/packages/std/src/ado_base/operators.rs](andromeda-core/packages/std/src/ado_base/operators.rs)
- [andromeda-core/packages/std/src/ado_base/ownership.rs](andromeda-core/packages/std/src/ado_base/ownership.rs)
- [andromeda-core/packages/std/src/ado_base/permissioning.rs](andromeda-core/packages/std/src/ado_base/permissioning.rs)
- [andromeda-core/packages/std/src/ado_base/version.rs](andromeda-core/packages/std/src/ado_base/version.rs)
- [andromeda-core/packages/std/src/ado_base/withdraw.rs](andromeda-core/packages/std/src/ado_base/withdraw.rs)
- [andromeda-core/packages/std/src/ado_contract/app.rs](andromeda-core/packages/std/src/ado_contract/app.rs)
- [andromeda-core/packages/std/src/ado_contract/execute.rs](andromeda-core/packages/std/src/ado_contract/execute.rs)
- [andromeda-core/packages/std/src/ado_contract/instantiate.rs](andromeda-core/packages/std/src/ado_contract/instantiate.rs)
- [andromeda-core/packages/std/src/ado_contract/mod.rs](andromeda-core/packages/std/src/ado_contract/mod.rs)
- [andromeda-core/packages/std/src/ado_contract/modules/execute.rs](andromeda-core/packages/std/src/ado_contract/modules/execute.rs)
- [andromeda-core/packages/std/src/ado_contract/modules/mod.rs](andromeda-core/packages/std/src/ado_contract/modules/mod.rs)
- [andromeda-core/packages/std/src/ado_contract/modules/query.rs](andromeda-core/packages/std/src/ado_contract/modules/query.rs)
- [andromeda-core/packages/std/src/ado_contract/ownership.rs](andromeda-core/packages/std/src/ado_contract/ownership.rs)
- [andromeda-core/packages/std/src/ado_contract/permissioning.rs](andromeda-core/packages/std/src/ado_contract/permissioning.rs)
- [andromeda-core/packages/std/src/ado_contract/query.rs](andromeda-core/packages/std/src/ado_contract/query.rs)
- [andromeda-core/packages/std/src/ado_contract/state.rs](andromeda-core/packages/std/src/ado_contract/state.rs)
- [andromeda-core/packages/std/src/ado_contract/withdraw.rs](andromeda-core/packages/std/src/ado_contract/withdraw.rs)
- [andromeda-core/packages/std/src/amp/addresses.rs](andromeda-core/packages/std/src/amp/addresses.rs)
- [andromeda-core/packages/std/src/amp/messages.rs](andromeda-core/packages/std/src/amp/messages.rs)
- [andromeda-core/packages/std/src/amp/mod.rs](andromeda-core/packages/std/src/amp/mod.rs)
- [andromeda-core/packages/std/src/amp/recipient.rs](andromeda-core/packages/std/src/amp/recipient.rs)
- [andromeda-core/packages/std/src/common/context.rs](andromeda-core/packages/std/src/common/context.rs)
- [andromeda-core/packages/std/src/common/expiration.rs](andromeda-core/packages/std/src/common/expiration.rs)
- [andromeda-core/packages/std/src/common/mod.rs](andromeda-core/packages/std/src/common/mod.rs)
- [andromeda-core/packages/std/src/common/queries.rs](andromeda-core/packages/std/src/common/queries.rs)
- [andromeda-core/packages/std/src/common/rates.rs](andromeda-core/packages/std/src/common/rates.rs)
- [andromeda-core/packages/std/src/common/response.rs](andromeda-core/packages/std/src/common/response.rs)
- [andromeda-core/packages/std/src/common/withdraw.rs](andromeda-core/packages/std/src/common/withdraw.rs)
- [andromeda-core/packages/std/src/lib.rs](andromeda-core/packages/std/src/lib.rs)


