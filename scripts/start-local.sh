#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status
set -o pipefail  # Pipe failure will be considered as command failure

# delete ledger directory
rm -rf ./test-ledger

# Start solana-test-validator with custom programs and token accounts
solana-test-validator --reset --ledger ./test-ledger \
  --url mainnet-beta \
  --bpf-program 43vcPfe8ThRLwfJqhXoM2KwqmpqQK1wCrfvZsxrULsbQ ./deps/s_controller.so \
  --bpf-program 3LqXTGs1UtPaFPtQG8WDV6a6KyeXPrajhq7yjSvAGQiY ./deps/flat_fee.so \
  --bpf-program 1idUSy4MGGKyKhvjSnGZ6Zc7Q4eKQcibym4BkEEw9KR ./deps/lido_calculator.so \
  --bpf-program sp1V4h2gWorkGhVcazBc22Hfo2f5sd7jcjT4EDPrWFF ./deps/spl_calculator.so \
  --bpf-program mare3SCyfZkAndpBRBeonETmkCCB3TJTTrz8ZN2dnhP ./deps/marinade_calculator.so \
  \
  --account CPk5V2ZqhLwYSguYT2dmrELuvNVskZhqMXJPSezusjQL ./deps/configs/pre-fund.json \
  --account BG4gEQXRWBVJmcE56Jc6UoL8nUfujvRo6r4dJP8wSLsW ./deps/configs/pre-fund.json \
  --account 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq ./deps/configs/pre-fund.json \
  \
  --clone 7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj \
  --clone jupSoLaHXQiZZTSfEWMTRRgpnyFm8f6sZdosWBjx93v \
  --clone BPSoLzmLQn47EP5aa7jmFngRL8KC3TWAeAwXwZD8ip3P \
  --clone Dso1bDeDjCQxTrWHqUUi63oBvV7Mdm6WaobLbQ7gnPQ \
  --clone mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So \
  --clone 5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm \
  --clone J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn \
  --clone he1iusmfkpAdwvxLNGV8Y1iSbj4rUy6yMhEA3fotn9A \
  \
  --account CdYEjJJ3TksiAcvUxD6Bzo1V6RahhCyaRpaTMctdGFFK ./deps/configs/token-account-jupsol.json \
  --account 4qnTdFvjH5aTB5DL3MS6JTHTNNoa3b6UieugLcVjgApC ./deps/configs/token-account-bpsol.json \
  --account 5KLgeXCc87m9mV3HV7a4dzNEbBEQv85JzQ5yEqJP3dW8 ./deps/configs/token-account-dsol.json \
  --account 2xcPzF3CXtdLiDWjW1ieyqJvrQo34GkotCzakN9nf3E4 ./deps/configs/token-account-msol.json \
  --account EjQcpXwQtrJqfAJdR34HZTWtHNTFMWshx7XWf5BZdhs8 ./deps/configs/token-account-INF.json \
  --account 7eJ7hWfgHCF5jTAGRYngHf87CLxEb4vFJ33Hi3JhbLgV ./deps/configs/token-account-jitosol.json \
  --account 6MjHYCypPB7BFooYjth7ZoNM9rVXwTNCjSSSvkvtA6GZ ./deps/configs/token-account-hsol.json \
  \
  --clone-upgradeable-program wsoGmxQLSvwWpuaidCApxN5kEowLe2HLQLJhCQnj4bE \
  \
  --clone-upgradeable-program CrX7kMhLC3cSsXJdT7JDgqrRVWGnUpX3gfEfxxU2NVLi \
  --clone 49Yi1TKkNyYjPAFdR9LBvoHcUjuPX4Df5T5yv39w2XTn \
  \
  --clone-upgradeable-program MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD \
  --clone 8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC \
  \
  --clone-upgradeable-program SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy \
  \
  --clone-upgradeable-program SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY \
  \
  --clone-upgradeable-program SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn \
  \
  --clone J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn \
  --clone Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb

# lido_calculator:           1idUSy4MGGKyKhvjSnGZ6Zc7Q4eKQcibym4BkEEw9KR
# lido_calculator_data:      HkuX8A8Q6XQjimK3LP6vfhAZrm4UJgUZTRR7uYbzj3FH
# lido_calculator state:     7Dv8K2G3DqfkNNdPDx6qaQKmzGQu18fg6S7AjRnew6aX
# spl_calculator:            sp1V4h2gWorkGhVcazBc22Hfo2f5sd7jcjT4EDPrWFF
# spl_calculator_data:       26GWfXL2eYH8AtX9b6Uw8PBrjLu98sWvc9wrfxLj4kLF
# spl_calculator state:      7orJ4kDhn1Ewp54j29tBzUWDFGhyimhYi7sxybZcphHd
# marinade_calculator:       mare3SCyfZkAndpBRBeonETmkCCB3TJTTrz8ZN2dnhP
# marinade_calculator_data:  3E5A8HNuvJJKYa58TZddiVMnc8ZTsRVw4pmrww3xgcQ5
# marinade_calculator state: FMbUjYFtqgm4Zfpg7MguZp33RQ3tvkd22NgaCCAs3M6E
# wsol_calculator:           wsoGmxQLSvwWpuaidCApxN5kEowLe2HLQLJhCQnj4bE
# wsol_calculator_data:      JCiy5grhSYeceD5FAd9AKQa9p2t7vXe6zC3sUDVjWMrx

# stsol:                     7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj
# msol:                      mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So
# wsol:                      So11111111111111111111111111111111111111112

# lido_program:              CrX7kMhLC3cSsXJdT7JDgqrRVWGnUpX3gfEfxxU2NVLi
# lido_program_progdata:     CHZNLhDXKrsXBmmv947RFciquwBsn2NdABmhpxoX3wgZ
# lido_state:                49Yi1TKkNyYjPAFdR9LBvoHcUjuPX4Df5T5yv39w2XTn

# marinade_program:          MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD
# marinade_program_progdata: 4PQH9YmfuKrVyZaibkLYpJZPv2FPaybhq2GAuBcWMSBf
# marinade_state:            8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC

# spl_stake_pool_program:    SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy
# spl_stake_pool_program_progdata: EmiU8AQkB2sswTxVB6aCmsAJftoowZGGDXuytm6X65R3

# sanctum_spl_stake_pool_program: SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY
# sanctum_spl_stake_pool_program_progdata: Cn5fegqLh8Fmvffisr4Wk3LmuaUgMMzTFfEuidpZFsvV

# sanctum_spl_multi_stake_pool_program: SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn
# sanctum_spl_multi_stake_pool_program_progdata: HxBTMuB7cFBPVWVJjTi9iBF8MPd7mfY1QnrrWfLAySFd

# ---
# jitosol:                   J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn
# jito_stake_pool:           Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb



# jupsol                     jupSoLaHXQiZZTSfEWMTRRgpnyFm8f6sZdosWBjx93v
# bpsol                      BPSoLzmLQn47EP5aa7jmFngRL8KC3TWAeAwXwZD8ip3P
# dsol                       Dso1bDeDjCQxTrWHqUUi63oBvV7Mdm6WaobLbQ7gnPQ
# msol                       mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So
# INF                        5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm
# jitosol                    J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn
# hsol                       he1iusmfkpAdwvxLNGV8Y1iSbj4rUy6yMhEA3fotn9A