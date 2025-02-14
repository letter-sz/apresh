use std::fs;
use std::path::Path;
use std::process::Command;

const LEDGER_WASM_URL: &str = "https://download.dfinity.systems/ic/35bfcadd0f2a474057e42393917b8b3ac269627a/canisters/ic-icrc1-ledger.wasm.gz";
const SCRIPTS_DIR: &str = "scripts";
const WASM_FILE: &str = "icrc1_ledger.wasm.gz";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let scripts_path = Path::new(SCRIPTS_DIR);
    if !scripts_path.exists() {
        fs::create_dir_all(scripts_path).expect("Failed to create scripts directory");
    }

    let wasm_path = scripts_path.join(WASM_FILE);
    if !wasm_path.exists() {
        println!("Downloading ICRC1 ledger WASM...");

        #[cfg(target_family = "unix")]
        {
            let status = Command::new("curl")
                .args(["-o", wasm_path.to_str().unwrap(), LEDGER_WASM_URL])
                .status()
                .expect("Failed to execute curl command");

            if !status.success() {
                panic!("Failed to download ICRC1 ledger WASM");
            }
        }

        println!("Successfully downloaded ICRC1 ledger WASM");
    }
}
