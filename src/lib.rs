//! The Cipher ParaTime.
use std::collections::BTreeMap;

use oasis_runtime_sdk::{
    self as sdk, modules,
    types::token::{BaseUnits, Denomination},
    Module as _, Version,
};

/// Configuration for the various modules.
pub struct Config;

impl module_contracts::Config for Config {
    type Accounts = modules::accounts::Module;
}

/// The Cipher ParaTime.
pub struct Runtime;

impl sdk::Runtime for Runtime {
    /// Version of the runtime.
    const VERSION: Version = sdk::version_from_cargo!();
    /// Current version of the global state (e.g. parameters). Any parameter updates should bump
    /// this version in order for the migrations to be executed.
    const STATE_VERSION: u32 = 1;

    type Modules = (
        // Core.
        modules::core::Module,
        // Accounts.
        modules::accounts::Module,
        // Consensus layer interface.
        modules::consensus::Module,
        // Consensus layer accounts.
        modules::consensus_accounts::Module<modules::accounts::Module, modules::consensus::Module>,
        // Rewards.
        modules::rewards::Module<modules::accounts::Module>,
        // Contracts.
        module_contracts::Module<Config>,
    );

    fn genesis_state() -> <Self::Modules as sdk::module::MigrationHandler>::Genesis {
        (
            modules::core::Genesis {
                parameters: modules::core::Parameters {
                    min_gas_price: {
                        let mut mgp = BTreeMap::new();
                        mgp.insert(Denomination::NATIVE, 0);
                        mgp
                    },
                    max_batch_gas: 50_000_000,
                    max_tx_signers: 1,
                    max_multisig_signers: 8,
                    gas_costs: modules::core::GasCosts {
                        tx_byte: 1,
                        auth_signature: 1_000,
                        auth_multisig_signer: 1_000,
                        callformat_x25519_deoxysii: 10_000,
                    },
                },
            },
            modules::accounts::Genesis {
                parameters: modules::accounts::Parameters {
                    gas_costs: modules::accounts::GasCosts { tx_transfer: 100 },
                    ..Default::default()
                },
                ..Default::default()
            },
            modules::consensus::Genesis {
                parameters: modules::consensus::Parameters {
                    // Consensus layer denomination is the native denomination of this runtime.
                    consensus_denomination: Denomination::NATIVE,
                },
            },
            modules::consensus_accounts::Genesis {
                parameters: modules::consensus_accounts::Parameters {
                    gas_costs: modules::consensus_accounts::GasCosts {
                        tx_deposit: 1_000,
                        tx_withdraw: 1_000,
                    },
                },
            },
            modules::rewards::Genesis {
                parameters: modules::rewards::Parameters {
                    schedule: modules::rewards::types::RewardSchedule {
                        steps: vec![modules::rewards::types::RewardStep {
                            until: 20_000,
                            amount: BaseUnits::new(10_000_000_000, Denomination::NATIVE),
                        }],
                    },
                    participation_threshold_numerator: 3,
                    participation_threshold_denominator: 4,
                },
            },
            module_contracts::Genesis {
                parameters: module_contracts::Parameters {
                    max_code_size: 256 * 1024, // 256 KiB
                    max_stack_size: 60 * 1024, // 60 KiB
                    max_memory_pages: 20,      // 1280 KiB

                    max_subcall_depth: 8,
                    max_subcall_count: 16,

                    max_result_size_bytes: 1024, // 1 KiB
                    max_query_size_bytes: 1024,  // 1 KiB
                    max_storage_key_size_bytes: 64,
                    max_storage_value_size_bytes: 16 * 1024, // 16 KiB

                    gas_costs: module_contracts::GasCosts {
                        tx_upload: 1_000,
                        tx_upload_per_byte: 1,
                        tx_instantiate: 1_000,
                        tx_call: 1_000,
                        tx_upgrade: 1_000,

                        subcall_dispatch: 100,

                        wasm_storage_get_base: 20,
                        wasm_storage_insert_base: 20,
                        wasm_storage_remove_base: 20,
                        wasm_storage_key_byte: 1,
                        wasm_storage_value_byte: 1,
                        wasm_env_query_base: 10,
                    },
                },
            },
        )
    }

    fn migrate_state<C: sdk::Context>(ctx: &mut C) {
        // State migration from v0 state by copying over parameters from updated genesis state.
        let genesis = Self::genesis_state();

        // Core.
        modules::core::Module::set_params(ctx.runtime_state(), genesis.0.parameters);
        // Rewards.
        modules::rewards::Module::<modules::accounts::Module>::set_params(
            ctx.runtime_state(),
            genesis.4.parameters,
        );
        // Contracts.
        module_contracts::Module::<Config>::set_params(ctx.runtime_state(), genesis.5.parameters);
    }
}
