//! The Cipher ParaTime.
use std::collections::{BTreeMap, BTreeSet};

use oasis_runtime_sdk::{
    self as sdk,
    crypto::signature::PublicKey,
    modules,
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
        // Bridge.
        bridge::Module<modules::accounts::Module>,
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
            bridge::Genesis {
                parameters: bridge::Parameters {
                    witnesses: vec![
                        PublicKey::Secp256k1("ApGjeGS0wK0iA7rcAtMwKpJv5By8nMFFWllkGd/RTOiN".into()), // Oasis Protocol Foundation
                        PublicKey::Secp256k1("Ait1frjDUZSSjafXB1ahTLhV8H/rGlq7k94i6AyPMg0i".into()), // cdot
                    ],
                    threshold: 2,
                    local_denominations: {
                        let mut ld = BTreeSet::new();
                        ld.insert(Denomination::NATIVE);
                        ld
                    },
                    remote_denominations: {
                        let mut rd = BTreeMap::new();
                        rd.insert(
                            "oETH".parse().unwrap(),
                            "0000000000000000000000000000000000000000".into(),
                        );
                        rd.insert(
                            "oUSDC".parse().unwrap(),
                            "07865c6e87b9f70255377e024ace6630c1eaa37f".into(),
                        );
                        rd.insert(
                            "oUSDT".parse().unwrap(),
                            "471297b3cb1e9f71661e7a92ced4b4b2b6e5b4a2".into(),
                        );
                        rd.insert(
                            "oDAI".parse().unwrap(),
                            "829de23e270519c5978e350deb0ec5ef8cd6af28".into(),
                        );
                        rd.insert(
                            "oLINK".parse().unwrap(),
                            "d91633cd725f815085cce509dd6e0f09210a0400".into(),
                        );
                        rd.insert(
                            "oWETH".parse().unwrap(),
                            "e5a393784df17d9f9f679c013257e06e72c37c0a".into(),
                        );
                        rd.insert(
                            "oWBTC".parse().unwrap(),
                            "f76eb8c27df517c25b5d5f91498b5c6203bfe55a".into(),
                        );
                        rd
                    },
                },
            },
        )
    }
}
