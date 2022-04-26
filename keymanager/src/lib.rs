//! The Cipher key manager.
use std::collections::HashSet;

use oasis_core_keymanager_api_common::TrustedPolicySigners;

/// Determine whether the build is for Testnet.
///
/// If the crate version has a pre-release component (e.g. 2.4.0-testnet) then the build is
/// classified as Testnet. If there is no such component (e.g. 2.4.0) then it is classified as
/// Mainnet.
const fn is_testnet() -> bool {
    !env!("CARGO_PKG_VERSION_PRE").is_empty()
}

/// Trusted key manager policy signer set.
pub fn trusted_policy_signers() -> TrustedPolicySigners {
    if is_testnet() {
        // Testnet.
        TrustedPolicySigners {
            signers: {
                let mut signers = HashSet::new();
                signers.insert(
                    "c37cbd0345965fda84fbaa372a01fc840b7b66eebfeb66dfdd35bb3e801f2cf3".into(),
                );
                signers.insert(
                    "df8ca9fc78ce2c01f8217e8ce7aa582e8952545f412426fe07d42ca119e12166".into(),
                );
                signers.insert(
                    "b27b3d0245d4cbd78be8e04e473f36abee350fcfbc438000313db1bb06117a43".into(),
                );
                signers
            },
            threshold: 2,
        }
    } else {
        // Mainnet.
        panic!("key manager policy signer set not defined for Mainnet");
    }
}
