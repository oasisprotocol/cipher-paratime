//! The key manager for the Cipher ParaTime.
use cipher_keymanager::trusted_policy_signers;
use oasis_core_keymanager_lib::keymanager::new_keymanager;
use oasis_core_runtime::{common::version::Version, config::Config, version_from_cargo};

/// Entrypoint.
pub fn main() {
    // Create a new Oasis Core key manager instance using the specified trusted signers.
    let init = new_keymanager(trusted_policy_signers());

    // Start the runtime.
    oasis_core_runtime::start_runtime(
        init,
        Config {
            version: version_from_cargo!(),
            ..Default::default()
        },
    );
}
