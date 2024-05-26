
# Andromeda ADO  contest details

- Join [Sherlock Discord](https://discord.gg/MABEWyASkp)
- Submit findings using the issue page in your private contest repo (label issues as med or high)
- [Read for more details](https://docs.sherlock.xyz/audits/watsons)

# Q&A

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


