//! The Cipher ParaTime.
use std::collections::{BTreeMap, BTreeSet};

#[cfg(target_env = "sgx")]
use oasis_runtime_sdk::core::consensus::verifier::TrustRoot;
use oasis_runtime_sdk::{
    self as sdk, config,
    core::common::crypto::signature::PublicKey,
    keymanager::TrustedPolicySigners,
    modules,
    types::token::{BaseUnits, Denomination},
    Module as _, Version,
};
use once_cell::unsync::Lazy;

/// Configuration for the various modules.
pub struct Config;

/// Determine whether the build is for Testnet.
///
/// If the crate version has a pre-release component (e.g. 2.4.0-testnet) then the build is
/// classified as Testnet. If there is no such component (e.g. 2.4.0) then it is classified as
/// Mainnet.
#[cfg_attr(not(target_env = "sgx"), allow(unused))]
const fn is_testnet() -> bool {
    !env!("CARGO_PKG_VERSION_PRE").is_empty()
}

/// Determine state version on weather the build is for Testnet or Mainnet.
#[allow(clippy::if_same_then_else)]
const fn state_version() -> u32 {
    if is_testnet() {
        // Testnet.
        6
    } else {
        // Mainnet.
        2
    }
}

impl modules::core::Config for Config {
    /// Default local minimum gas price configuration that is used in case no overrides are set in
    /// local per-node configuration.
    const DEFAULT_LOCAL_MIN_GAS_PRICE: Lazy<BTreeMap<Denomination, u128>> =
        Lazy::new(|| BTreeMap::from([(Denomination::NATIVE, 5)]));

    /// Methods which are exempt from minimum gas price requirements.
    const MIN_GAS_PRICE_EXEMPT_METHODS: Lazy<BTreeSet<&'static str>> =
        Lazy::new(|| BTreeSet::from(["consensus.Deposit"]));
}

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
    const STATE_VERSION: u32 = state_version();

    /// Schedule control configuration.
    const SCHEDULE_CONTROL: config::ScheduleControl = config::ScheduleControl {
        initial_batch_size: 50,
        batch_size: 50,
        min_remaining_gas: 30_000, // accounts.Transfer method calls.
        max_tx_count: 1_000,       // Consistent with runtime descriptor.
    };

    type Core = modules::core::Module<Config>;

    #[allow(clippy::type_complexity)]
    type Modules = (
        // Core.
        modules::core::Module<Config>,
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

    fn trusted_policy_signers() -> Option<TrustedPolicySigners> {
        #[allow(clippy::partialeq_to_none)]
        if option_env!("OASIS_UNSAFE_SKIP_KM_POLICY") == Some("1") {
            return Some(TrustedPolicySigners::default());
        }
        let tps = keymanager::trusted_policy_signers();
        // The `keymanager` crate may use a different version of `oasis_core`
        // so we need to convert the `TrustedPolicySigners` between the versions.
        Some(TrustedPolicySigners {
            signers: tps.signers.into_iter().map(|s| PublicKey(s.0)).collect(),
            threshold: tps.threshold,
        })
    }

    #[cfg(target_env = "sgx")]
    fn consensus_trust_root() -> Option<TrustRoot> {
        if is_testnet() {
            // Testnet.
            Some(TrustRoot {
                height: 13670553,
                hash: "7e0e12dcdaa9e8a83e27799c03c873a0a2fc720bcef044992578a936ac7da2a2".into(),
                runtime_id: "0000000000000000000000000000000000000000000000000000000000000000"
                    .into(),
                chain_context: "50304f98ddb656620ea817cc1446c401752a05a249b36c9b90dba4616829977a"
                    .to_string(),
            })
        } else {
            // Mainnet.
            Some(TrustRoot {
                height: 12282779,
                hash: "c132606dc870fbf386cb7a84f8fb033c4911f8dd0b235efa22219b2c6cf7bf35".into(),
                runtime_id: "000000000000000000000000000000000000000000000000e199119c992377cb"
                    .into(),
                chain_context: "b11b369e0da5bb230b220127f5e7b242d385ef8c6f54906243f30af63c815535"
                    .to_string(),
            })
        }
    }

    #[allow(clippy::borrow_interior_mutable_const)]
    fn genesis_state() -> <Self::Modules as sdk::module::MigrationHandler>::Genesis {
        (
            modules::core::Genesis {
                parameters: modules::core::Parameters {
                    min_gas_price: <Config as modules::core::Config>::DEFAULT_LOCAL_MIN_GAS_PRICE
                        .clone(),
                    dynamic_min_gas_price: modules::core::DynamicMinGasPrice {
                        enabled: false,
                        target_block_gas_usage_percentage: 50,
                        min_price_max_change_denominator: 8,
                    },
                    max_batch_gas: 1_000_000_000,
                    max_tx_size: 1024 * 1024,
                    max_tx_signers: 1,
                    max_multisig_signers: 8,
                    gas_costs: modules::core::GasCosts {
                        tx_byte: 20,
                        auth_signature: 100_000,
                        auth_multisig_signer: 100_000,
                        callformat_x25519_deoxysii: 50_000,
                    },
                },
            },
            modules::accounts::Genesis {
                parameters: modules::accounts::Parameters {
                    gas_costs: modules::accounts::GasCosts {
                        tx_transfer: 30_000,
                    },
                    denomination_infos: {
                        let mut denomination_infos = BTreeMap::new();
                        denomination_infos.insert(
                            Denomination::NATIVE,
                            modules::accounts::types::DenominationInfo { decimals: 9 },
                        );
                        denomination_infos
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            modules::consensus::Genesis {
                parameters: modules::consensus::Parameters {
                    // Consensus layer denomination is the native denomination of this runtime.
                    consensus_denomination: Denomination::NATIVE,
                    consensus_scaling_factor: 1,
                    // Minimum delegation amount that matches the consensus layer.
                    min_delegate_amount: 100_000_000_000,
                },
            },
            modules::consensus_accounts::Genesis {
                parameters: modules::consensus_accounts::Parameters {
                    gas_costs: modules::consensus_accounts::GasCosts {
                        tx_deposit: 4_000_000,
                        tx_withdraw: 4_000_000,
                        tx_delegate: 4_000_000,
                        tx_undelegate: 8_000_000,

                        store_receipt: 1_300_000,
                        take_receipt: 1_000_000,
                    },
                    disable_delegate: false,
                    disable_undelegate: false,
                    disable_deposit: false,
                    disable_withdraw: false,
                },
            },
            modules::rewards::Genesis {
                parameters: modules::rewards::Parameters {
                    schedule: modules::rewards::types::RewardSchedule {
                        steps: vec![modules::rewards::types::RewardStep {
                            until: 26_700,
                            amount: BaseUnits::new(10_000_000_000, Denomination::NATIVE),
                        }],
                    },
                    participation_threshold_numerator: 3,
                    participation_threshold_denominator: 4,
                },
            },
            module_contracts::Genesis {
                parameters: module_contracts::Parameters {
                    max_memory_pages: 20, // 1280 KiB

                    ..Default::default()
                },
            },
        )
    }

    fn migrate_state<C: sdk::Context>(_ctx: &mut C) {
        // State migration from by copying over parameters from updated genesis state.
        let genesis = Self::genesis_state();

        // Core.
        modules::core::Module::<Config>::set_params(genesis.0.parameters);
        // Accounts.
        modules::accounts::Module::set_params(genesis.1.parameters);
        // Consensus.
        modules::consensus::Module::set_params(genesis.2.parameters);
        // Consensus accounts.
        modules::consensus_accounts::Module::<modules::accounts::Module, modules::consensus::Module>::set_params(
            genesis.3.parameters,
        );
        // Rewards.
        modules::rewards::Module::<modules::accounts::Module>::set_params(genesis.4.parameters);
        // Contracts.
        module_contracts::Module::<Config>::set_params(genesis.5.parameters);
    }
}
