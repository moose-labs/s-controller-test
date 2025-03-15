use std::{
    process::{Child, Command, Stdio},
    time::Duration,
};

use moose_utils::{paths::resolve_path, result::Result};
use tokio::time::sleep;

/// Starts the solana-test-validator with the given ledger directory,
/// a dynamic port range, and returns the spawned child process.
///
/// This function waits for 5 seconds to allow the validator to start.
pub async fn start_test_validator() -> Result<Child> {
    let child = Command::new("solana-test-validator")
        .args(&build_test_validator_args())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Wait a bit for the validator to start up.
    sleep(Duration::from_secs(5)).await;

    Ok(child)
}

fn resolve_workspace_path(path: &str) -> String {
    resolve_path(path).to_str().unwrap().to_string()
}

fn build_test_validator_args() -> Vec<String> {
    // Convert paths to strings.
    let ledger_path = resolve_workspace_path("test-ledger");
    let s_controller_so = resolve_workspace_path("deps/s_controller.so");
    let flat_fee_so = resolve_workspace_path("deps/flat_fee.so");
    let lido_calculator_so = resolve_workspace_path("deps/lido_calculator.so");
    let marinade_calculator_so = resolve_workspace_path("deps/marinade_calculator.so");
    let spl_calculator_so = resolve_workspace_path("deps/spl_calculator.so");
    let pre_fund_json = resolve_workspace_path("deps/configs/pre-fund.json");
    let usdc_json = resolve_workspace_path("deps/configs/usdc.json");
    let wbtc_json = resolve_workspace_path("deps/configs/wbtc.json");

    // Build the vector of arguments.
    vec![
        "--reset".into(),
        "--ledger".into(),
        ledger_path,
        "--url".into(),
        "mainnet-beta".into(),
        "--bpf-program".into(),
        "43vcPfe8ThRLwfJqhXoM2KwqmpqQK1wCrfvZsxrULsbQ".into(), // s_controller program
        s_controller_so,
        "--bpf-program".into(),
        "3LqXTGs1UtPaFPtQG8WDV6a6KyeXPrajhq7yjSvAGQiY".into(), // flat_fee program
        flat_fee_so,
        "--bpf-program".into(),
        "1idUSy4MGGKyKhvjSnGZ6Zc7Q4eKQcibym4BkEEw9KR".into(), // lido_calculator
        lido_calculator_so.into(),
        "--bpf-program".into(),
        "sp1V4h2gWorkGhVcazBc22Hfo2f5sd7jcjT4EDPrWFF".into(), // spl_calculator
        spl_calculator_so.into(),
        "--bpf-program".into(),
        "mare3SCyfZkAndpBRBeonETmkCCB3TJTTrz8ZN2dnhP".into(), // marinade_calculator
        marinade_calculator_so.into(),
        // accounts
        "--account".into(),
        "BG4gEQXRWBVJmcE56Jc6UoL8nUfujvRo6r4dJP8wSLsW".into(), // pre-fund account (auth)
        pre_fund_json.clone(),
        "--account".into(),
        "54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq".into(), // pre-fund account (user)
        pre_fund_json,
        "--account".into(),
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".into(), // USDC mint account
        usdc_json,
        "--account".into(),
        "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh".into(), // WBTC account;
        wbtc_json,
        // Clones from live network
        "--clone-upgradeable-program".into(),
        "wsoGmxQLSvwWpuaidCApxN5kEowLe2HLQLJhCQnj4bE".into(), // wsol_calculator
        "--clone".into(),
        "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj".into(), // stsol mint
        "--clone".into(),
        "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So".into(), // msol mint
        "--clone-upgradeable-program".into(),
        "CrX7kMhLC3cSsXJdT7JDgqrRVWGnUpX3gfEfxxU2NVLi".into(), // lido_program
        "--clone".into(),
        "49Yi1TKkNyYjPAFdR9LBvoHcUjuPX4Df5T5yv39w2XTn".into(), // lido_state
        "--clone-upgradeable-program".into(),
        "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD".into(), // marinade_program
        "--clone".into(),
        "8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC".into(), // marinade_state
        "--clone-upgradeable-program".into(),
        "SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy".into(), // spl_stake_pool_program
        "--clone-upgradeable-program".into(),
        "SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY".into(), // sanctum_spl_stake_pool_program
        "--clone-upgradeable-program".into(),
        "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn".into(), // sanctum_spl_multi_stake_pool_program
        "--clone".into(),
        "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn".into(), // jitosol mint
        "--clone".into(),
        "Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb".into(), // jito_stake_pool
        "--clone-upgradeable-program".into(),
        "f1tUoNEKrDp1oeGn4zxr7bh41eN6VcfHjfrL3ZqQday".into(), // flat_fee program (live)
    ]
}
