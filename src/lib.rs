//! The Cipher ParaTime.
use oasis_runtime_sdk::{
    self as sdk, modules,
    types::token::{BaseUnits, Denomination},
    Version,
};

/// The Cipher ParaTime.
pub struct Runtime;

impl sdk::Runtime for Runtime {
    const VERSION: Version = sdk::version_from_cargo!();

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
    );

    fn genesis_state() -> <Self::Modules as sdk::module::MigrationHandler>::Genesis {
        (
            modules::core::Genesis {
                parameters: modules::core::Parameters {
                    max_batch_gas: 1_000_000,
                    max_tx_signers: 1,
                    max_multisig_signers: 8,
                    gas_costs: modules::core::GasCosts {
                        auth_signature: 1_000,
                        auth_multisig_signer: 1_000,
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
                            // TODO: This is based on the current testnet epoch.
                            until: 7418 + 24 * 60, // Roughly 60 days.
                            // TODO: Define proper reward schedule.
                            amount: BaseUnits::new(10.into(), Denomination::NATIVE),
                        }],
                    },
                    participation_threshold_numerator: 3,
                    participation_threshold_denominator: 4,
                },
            },
        )
    }
}
