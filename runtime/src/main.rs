use oasis_runtime_sdk::Runtime;

#[cfg(all(target_env = "sgx", target_vendor = "fortanix"))]
mod abort;

fn main() {
    cipher_paratime::Runtime::start();
}
