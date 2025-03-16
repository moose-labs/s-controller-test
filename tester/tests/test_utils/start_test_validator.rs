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
    let ata_bpsol = resolve_workspace_path("deps/configs/token-account-bpsol.json");
    let ata_dsol = resolve_workspace_path("deps/configs/token-account-dsol.json");
    let ata_hsol = resolve_workspace_path("deps/configs/token-account-hsol.json");
    let ata_inf = resolve_workspace_path("deps/configs/token-account-INF.json");
    let ata_jitosol = resolve_workspace_path("deps/configs/token-account-jitosol.json");
    let ata_jupsol = resolve_workspace_path("deps/configs/token-account-jupsol.json");
    let ata_msol = resolve_workspace_path("deps/configs/token-account-msol.json");

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
        // fund accounts
        "--account".into(),
        "CPk5V2ZqhLwYSguYT2dmrELuvNVskZhqMXJPSezusjQL".into(), // pre-fund account (local-auth)
        pre_fund_json.clone(),
        "--account".into(),
        "BG4gEQXRWBVJmcE56Jc6UoL8nUfujvRo6r4dJP8wSLsW".into(), // pre-fund account (user2)
        pre_fund_json.clone(),
        "--account".into(),
        "54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq".into(), // pre-fund account (user1)
        pre_fund_json,
        //
        // Clones mints from live network
        //
        "--clone".into(),
        "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj".into(), // stsol mint
        "--clone".into(),
        "jupSoLaHXQiZZTSfEWMTRRgpnyFm8f6sZdosWBjx93v".into(), // jupsol mint
        "--clone".into(),
        "BPSoLzmLQn47EP5aa7jmFngRL8KC3TWAeAwXwZD8ip3P".into(), // bpsol mint
        "--clone".into(),
        "Dso1bDeDjCQxTrWHqUUi63oBvV7Mdm6WaobLbQ7gnPQ".into(), // dosl mint
        "--clone".into(),
        "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So".into(), // msol mint
        "--clone".into(),
        "5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm".into(), // INF mint
        "--clone".into(),
        "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn".into(), // jito mint
        "--clone".into(),
        "he1iusmfkpAdwvxLNGV8Y1iSbj4rUy6yMhEA3fotn9A".into(), // hsol mint
        //
        // prime token accounts
        //
        "--account".into(),
        "CdYEjJJ3TksiAcvUxD6Bzo1V6RahhCyaRpaTMctdGFFK".into(), // ata jupsol
        ata_jupsol.into(),
        "--account".into(),
        "4qnTdFvjH5aTB5DL3MS6JTHTNNoa3b6UieugLcVjgApC".into(), // ata bpsol
        ata_bpsol.into(),
        "--account".into(),
        "5KLgeXCc87m9mV3HV7a4dzNEbBEQv85JzQ5yEqJP3dW8".into(), // ata dsol
        ata_dsol.into(),
        "--account".into(),
        "2xcPzF3CXtdLiDWjW1ieyqJvrQo34GkotCzakN9nf3E4".into(), // ata msol
        ata_msol.into(),
        "--account".into(),
        "EjQcpXwQtrJqfAJdR34HZTWtHNTFMWshx7XWf5BZdhs8".into(), // ata INF
        ata_inf.into(),
        "--account".into(),
        "7eJ7hWfgHCF5jTAGRYngHf87CLxEb4vFJ33Hi3JhbLgV".into(), // ata jitosol
        ata_jitosol.into(),
        "--account".into(),
        "6MjHYCypPB7BFooYjth7ZoNM9rVXwTNCjSSSvkvtA6GZ".into(), // ata hsol
        ata_hsol.into(),
        //
        // Clones programs/accounts from live network
        //
        "--clone-upgradeable-program".into(),
        "wsoGmxQLSvwWpuaidCApxN5kEowLe2HLQLJhCQnj4bE".into(), // wsol_calculator
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
        "Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb".into(), // jito_stake_pool
        "--clone-upgradeable-program".into(),
        "f1tUoNEKrDp1oeGn4zxr7bh41eN6VcfHjfrL3ZqQday".into(), // flat_fee program (live)
    ]
}
