//! The Cipher ParaTime.
use std::collections::{BTreeMap, BTreeSet};

#[cfg(not(feature = "debug-mock-sgx"))]
use oasis_runtime_sdk::core::common::crypto::signature::PublicKey;
#[cfg(target_env = "sgx")]
use oasis_runtime_sdk::core::consensus::verifier::TrustRoot;
use oasis_runtime_sdk::{
    self as sdk, config,
    keymanager::TrustedSigners,
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
        14
    } else {
        // Mainnet.
        8
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

impl module_contracts::Config for Config {}

impl modules::rofl::Config for Config {
    /// Gas cost of rofl.Create call.
    const GAS_COST_CALL_CREATE: u64 = 6_000_000;
    /// Gas cost of rofl.Update call.
    const GAS_COST_CALL_UPDATE: u64 = 6_000_000;
    /// Gas cost of rofl.Remove call.
    const GAS_COST_CALL_REMOVE: u64 = 600_000;
    /// Gas cost of rofl.Register call.
    const GAS_COST_CALL_REGISTER: u64 = 6_000_000;
    /// Gas cost of rofl.IsAuthorizedOrigin call.
    const GAS_COST_CALL_IS_AUTHORIZED_ORIGIN: u64 = 60_000;
    /// Gas cost of rofl.AuthorizedOriginNode call.
    const GAS_COST_CALL_AUTHORIZED_ORIGIN_NODE: u64 = 120_000;
    /// Gas cost of rofl.AuthorizedOriginEntity call.
    const GAS_COST_CALL_AUTHORIZED_ORIGIN_ENTITY: u64 = 120_000;
    /// Gas cost of rofl.OriginApp call.
    const GAS_COST_CALL_ORIGIN_APP: u64 = 60_000;
    /// Gas cost of rofl.StakeThresholds call.
    const GAS_COST_CALL_STAKE_THRESHOLDS: u64 = 10;
    /// Gas cost of rofl.DeriveKey call.
    const GAS_COST_CALL_DERIVE_KEY: u64 = 600_000;

    /// Amount of stake required for maintaining an application (100 ROSE/TEST).
    const STAKE_APP_CREATE: BaseUnits = BaseUnits::new(100_000_000_000, Denomination::NATIVE);

    /// Endorsement policy evaluator.
    type EndorsementPolicyEvaluator = (
        modules::rofl::policy::BasicEndorsementPolicyEvaluator,
        module_rofl_market::policy::ProviderEndorsementPolicyEvaluator,
    );
}

impl module_rofl_market::Config for Config {
    /// Module implementing the ROFL API.
    type Rofl = modules::rofl::Module<Config>;

    /// Gas cost of roflmarket.ProviderCreate call.
    const GAS_COST_CALL_PROVIDER_CREATE: u64 = 6_000_000;
    /// Gas cost of roflmarket.ProviderUpdate call.
    const GAS_COST_CALL_PROVIDER_UPDATE: u64 = 6_000_000;
    /// Gas cost of roflmarket.ProviderUpdateOffers call.
    const GAS_COST_CALL_PROVIDER_UPDATE_OFFERS_BASE: u64 = 6_000_000;
    /// Gas cost of each added offer in roflmarket.ProviderUpdateOffers call.
    const GAS_COST_CALL_PROVIDER_UPDATE_OFFERS_ADD: u64 = 600_000;
    /// Gas cost of each removed offer in roflmarket.ProviderUpdateOffers call.
    const GAS_COST_CALL_PROVIDER_UPDATE_OFFERS_RM: u64 = 60_000;
    /// Gas cost of roflmarket.ProviderRemove call.
    const GAS_COST_CALL_PROVIDER_REMOVE: u64 = 6_000_000;
    /// Gas cost of roflmarket.InstanceCreate call.
    const GAS_COST_CALL_INSTANCE_CREATE: u64 = 6_000_000;
    /// Gas cost of roflmarket.InstanceChangeAdmin call.
    const GAS_COST_CALL_INSTANCE_CHANGE_ADMIN: u64 = 600_000;
    /// Gas cost of roflmarket.InstanceAccept call.
    const GAS_COST_CALL_INSTANCE_ACCEPT_BASE: u64 = 600_000;
    /// Gas cost of each accepted instance in roflmarket.InstanceAccept call.
    const GAS_COST_CALL_INSTANCE_ACCEPT_INSTANCE: u64 = 600_000;
    /// Gas cost of roflmarket.InstanceTopUp call.
    const GAS_COST_CALL_INSTANCE_TOPUP: u64 = 600_000;
    /// Gas cost of roflmarket.InstanceUpdate call.
    const GAS_COST_CALL_INSTANCE_UPDATE_BASE: u64 = 600_000;
    /// Gas cost of each instance update in roflmarket.InstanceUpdate call.
    const GAS_COST_CALL_INSTANCE_UPDATE_INST: u64 = 600_000;
    /// Gas cost of roflmarket.InstanceCancel call.
    const GAS_COST_CALL_INSTANCE_CANCEL: u64 = 600_000;
    /// Gas cost of roflmarket.InstanceRemove call.
    const GAS_COST_CALL_INSTANCE_REMOVE: u64 = 600_000;
    /// Gas cost of roflmarket.InstanceExecuteCmds call.
    const GAS_COST_CALL_INSTANCE_EXECUTE_CMDS_BASE: u64 = 600_000;
    /// Gas cost of each command in roflmarket.InstanceExecuteCmds call.
    const GAS_COST_CALL_INSTANCE_EXECUTE_CMDS_CMD: u64 = 600_000;
    /// Gas cost of roflmarket.InstanceClaimPayment call.
    const GAS_COST_CALL_INSTANCE_CLAIM_PAYMENT_BASE: u64 = 600_000;
    /// Gas cost of each instance in roflmarket.InstanceClaimPayment call.
    const GAS_COST_CALL_INSTANCE_CLAIM_PAYMENT_INST: u64 = 600_000;

    /// Maximum time for a provider to accept an instance. If not accepted within this window, the
    /// instance may be cancelled and will be refunded.
    const MAX_INSTANCE_ACCEPT_TIME_SECONDS: u64 = 300;
    /// Maximum number of offers a provider can have.
    const MAX_PROVIDER_OFFERS: u64 = 64;
    /// Maximum number of queued instance commands.
    const MAX_QUEUED_INSTANCE_COMMANDS: u64 = 8;
    /// Maximum size of an instance command.
    const MAX_INSTANCE_COMMAND_SIZE: usize = 16 * 1024;

    /// Maximum number of metadata key-value pairs.
    const MAX_METADATA_PAIRS: usize = 64;
    /// Maximum metadata key size.
    const MAX_METADATA_KEY_SIZE: usize = 1024;
    /// Maximum metadata value size.
    const MAX_METADATA_VALUE_SIZE: usize = 16 * 1024;

    /// Amount of stake required for maintaining a provider (100 ROSE/TEST).
    const STAKE_PROVIDER_CREATE: BaseUnits = BaseUnits::new(100_000_000_000, Denomination::NATIVE);
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
    type Accounts = modules::accounts::Module;
    type FeeProxy = modules::rofl::Module<Config>;

    #[allow(clippy::type_complexity)]
    type Modules = (
        // Core.
        modules::core::Module<Config>,
        // Accounts.
        modules::accounts::Module,
        // Consensus layer interface.
        modules::consensus::Module,
        // Consensus layer accounts.
        modules::consensus_accounts::Module<modules::consensus::Module>,
        // Rewards.
        modules::rewards::Module,
        // ROFL.
        modules::rofl::Module<Config>,
        // Contracts.
        module_contracts::Module<Config>,
        // ROFL market.
        module_rofl_market::Module<Config>,
    );

    #[cfg(feature = "debug-mock-sgx")]
    fn trusted_signers() -> Option<TrustedSigners> {
        Some(TrustedSigners::unsafe_mock())
    }

    #[cfg(not(feature = "debug-mock-sgx"))]
    fn trusted_signers() -> Option<TrustedSigners> {
        let tps = keymanager::trusted_policy_signers();
        // The `keymanager` crate may use a different version of `oasis_core`
        // so we need to convert the `TrustedSigners` between the versions.
        Some(TrustedSigners {
            signers: tps.signers.into_iter().map(|s| PublicKey(s.0)).collect(),
            threshold: tps.threshold,
        })
    }

    #[cfg(target_env = "sgx")]
    fn consensus_trust_root() -> Option<TrustRoot> {
        if is_testnet() {
            // Testnet.
            Some(TrustRoot {
                height: 24544750,
                hash: "6b2b5e7990e0c8bee8035934bc52a83a16fe4bff4dbb81b1aa5a8f9409b2eafe".into(),
                runtime_id: "0000000000000000000000000000000000000000000000000000000000000000"
                    .into(),
                chain_context: "0b91b8e4e44b2003a7c5e23ddadb5e14ef5345c0ebcb3ddcae07fa2f244cab76"
                    .to_string(),
            })
        } else {
            // Mainnet.
            Some(TrustRoot {
                height: 22327937,
                hash: "01670e0af13fa4ceaeee8bd9672374d27a889b9304c44ab9acfd7f97663ee1b9".into(),
                runtime_id: "000000000000000000000000000000000000000000000000e199119c992377cb"
                    .into(),
                chain_context: "bb3d748def55bdfb797a2ac53ee6ee141e54cd2ab2dc2375f4a0703a178e6e55"
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
                        enabled: true,
                        target_block_gas_usage_percentage: 50,
                        min_price_max_change_denominator: 8,
                    },
                    max_batch_gas: 1_000_000_000,
                    max_tx_size: 1024 * 1024,
                    max_tx_signers: 3,
                    max_multisig_signers: 8,
                    gas_costs: modules::core::GasCosts {
                        tx_byte: 20,
                        storage_byte: 950,
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
                        BTreeMap::from([(
                            Denomination::NATIVE,
                            modules::accounts::types::DenominationInfo { decimals: 9 },
                        )])
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            modules::consensus::Genesis {
                parameters: modules::consensus::Parameters {
                    gas_costs: modules::consensus::GasCosts {
                        round_root: 300_000,
                    },
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

                        delegation: 700_000,
                        shares_to_tokens: 700_000,
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
            modules::rofl::Genesis {
                parameters: Default::default(),
                apps: vec![],
            },
            module_contracts::Genesis {
                parameters: module_contracts::Parameters {
                    max_memory_pages: 20, // 1280 KiB

                    ..Default::default()
                },
            },
            module_rofl_market::Genesis::default(),
        )
    }

    fn migrate_state<C: sdk::Context>(_ctx: &C) {
        // State migration from by copying over parameters from updated genesis state.
        let genesis = Self::genesis_state();

        // Core.
        modules::core::Module::<Config>::set_params(genesis.0.parameters);
        // Accounts.
        modules::accounts::Module::set_params(genesis.1.parameters);
        // Consensus.
        modules::consensus::Module::set_params(genesis.2.parameters);
        // Consensus accounts.
        modules::consensus_accounts::Module::<modules::consensus::Module>::set_params(
            genesis.3.parameters,
        );
        // Rewards.
        modules::rewards::Module::set_params(genesis.4.parameters);
        // ROFL.
        modules::rofl::Module::<Config>::set_params(genesis.5.parameters);
        // Contracts.
        module_contracts::Module::<Config>::set_params(genesis.6.parameters);
        // ROFL market.
        module_rofl_market::Module::<Config>::set_params(genesis.7.parameters);
    }
}
