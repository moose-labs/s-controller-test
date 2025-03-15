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
    let pre_fund_json = resolve_workspace_path("deps/configs/pre-fund.json");
    let usdc_json = resolve_workspace_path("deps/configs/usdc.json");
    let wbtc_json = resolve_workspace_path("deps/configs/wbtc.json");

    // You can continue similarly for other path-based arguments.

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
        // Clones:
        "--clone".into(),
        "f1tUoNEKrDp1oeGn4zxr7bh41eN6VcfHjfrL3ZqQday".into(), // flat_fee program (live)
        "--clone".into(),
        "1idUSy4MGGKyKhvjSnGZ6Zc7Q4eKQcibym4BkEEw9KR".into(), // lido_calculator
        // "--clone".into(),
        // "HkuX8A8Q6XQjimK3LP6vfhAZrm4UJgUZTRR7uYbzj3FH".into(), // lido_calculator_data
        "--clone".into(),
        "7Dv8K2G3DqfkNNdPDx6qaQKmzGQu18fg6S7AjRnew6aX".into(), // lido_calculator_state
        "--clone".into(),
        "sp1V4h2gWorkGhVcazBc22Hfo2f5sd7jcjT4EDPrWFF".into(), // spl_calculator
        // "--clone".into(),
        // "26GWfXL2eYH8AtX9b6Uw8PBrjLu98sWvc9wrfxLj4kLF".into(), // spl_calculator_data
        "--clone".into(),
        "7orJ4kDhn1Ewp54j29tBzUWDFGhyimhYi7sxybZcphHd".into(), // spl_calculator_state
        "--clone".into(),
        "mare3SCyfZkAndpBRBeonETmkCCB3TJTTrz8ZN2dnhP".into(), // marinade_calculator
        "--clone".into(),
        "3E5A8HNuvJJKYa58TZddiVMnc8ZTsRVw4pmrww3xgcQ5".into(), // marinade_calculator_data
        "--clone".into(),
        "FMbUjYFtqgm4Zfpg7MguZp33RQ3tvkd22NgaCCAs3M6E".into(), // marinade_calculator_state
        "--clone".into(),
        "wsoGmxQLSvwWpuaidCApxN5kEowLe2HLQLJhCQnj4bE".into(), // wsol_calculator
        // "--clone".into(),
        // "JCiy5grhSYeceD5FAd9AKQa9p2t7vXe6zC3sUDVjWMrx".into(), // wsol_calculator_data
        "--clone".into(),
        "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj".into(), // stsol mint
        "--clone".into(),
        "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So".into(), // msol mint
        "--clone".into(),
        "CrX7kMhLC3cSsXJdT7JDgqrRVWGnUpX3gfEfxxU2NVLi".into(), // lido_program
        "--clone".into(),
        "CHZNLhDXKrsXBmmv947RFciquwBsn2NdABmhpxoX3wgZ".into(), // lido_program_progdata
        "--clone".into(),
        "49Yi1TKkNyYjPAFdR9LBvoHcUjuPX4Df5T5yv39w2XTn".into(), // lido_state
        "--clone".into(),
        "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD".into(), // marinade_program
        "--clone".into(),
        "4PQH9YmfuKrVyZaibkLYpJZPv2FPaybhq2GAuBcWMSBf".into(), // marinade_program_progdata
        "--clone".into(),
        "8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC".into(), // marinade_state
        "--clone".into(),
        "SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy".into(), // spl_stake_pool_program
        "--clone".into(),
        "EmiU8AQkB2sswTxVB6aCmsAJftoowZGGDXuytm6X65R3".into(), // spl_stake_pool_program_progdata
        "--clone".into(),
        "SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY".into(), // sanctum_spl_stake_pool_program
        "--clone".into(),
        "Cn5fegqLh8Fmvffisr4Wk3LmuaUgMMzTFfEuidpZFsvV".into(), // sanctum_spl_stake_pool_program_progdata
        "--clone".into(),
        "SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn".into(), // sanctum_spl_multi_stake_pool_program
        "--clone".into(),
        "HxBTMuB7cFBPVWVJjTi9iBF8MPd7mfY1QnrrWfLAySFd".into(), // sanctum_spl_multi_stake_pool_program_progdata
        "--clone".into(),
        "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn".into(), // jitosol mint
        "--clone".into(),
        "Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb".into(), // jito_stake_pool
    ]
}

//     return args;
// }
