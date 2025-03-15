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
  \
  --account BG4gEQXRWBVJmcE56Jc6UoL8nUfujvRo6r4dJP8wSLsW ./deps/configs/pre-fund.json \
  --account 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq ./deps/configs/pre-fund.json \
  --account EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v ./deps/configs/usdc.json \
  --account 3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh ./deps/configs/wbtc.json \
  \
  --clone-upgradeable-program 1idUSy4MGGKyKhvjSnGZ6Zc7Q4eKQcibym4BkEEw9KR \
  --clone 7Dv8K2G3DqfkNNdPDx6qaQKmzGQu18fg6S7AjRnew6aX \
  \
  --clone-upgradeable-program sp1V4h2gWorkGhVcazBc22Hfo2f5sd7jcjT4EDPrWFF \
  --clone 7orJ4kDhn1Ewp54j29tBzUWDFGhyimhYi7sxybZcphHd \
  \
  --clone-upgradeable-program mare3SCyfZkAndpBRBeonETmkCCB3TJTTrz8ZN2dnhP \
  --clone FMbUjYFtqgm4Zfpg7MguZp33RQ3tvkd22NgaCCAs3M6E \
  \
  --clone-upgradeable-program wsoGmxQLSvwWpuaidCApxN5kEowLe2HLQLJhCQnj4bE \
  \
  --clone 7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj \
  --clone mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So \
  \
  --clone-upgradeable-program CrX7kMhLC3cSsXJdT7JDgqrRVWGnUpX3gfEfxxU2NVLi \
  --clone CHZNLhDXKrsXBmmv947RFciquwBsn2NdABmhpxoX3wgZ \
  --clone 49Yi1TKkNyYjPAFdR9LBvoHcUjuPX4Df5T5yv39w2XTn \
  \
  --clone-upgradeable-program MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD \
  --clone 4PQH9YmfuKrVyZaibkLYpJZPv2FPaybhq2GAuBcWMSBf \
  --clone 8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC \
  \
  --clone-upgradeable-program SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy \
  --clone EmiU8AQkB2sswTxVB6aCmsAJftoowZGGDXuytm6X65R3 \
  \
  --clone-upgradeable-program SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY \
  --clone Cn5fegqLh8Fmvffisr4Wk3LmuaUgMMzTFfEuidpZFsvV \
  \
  --clone-upgradeable-program SPMBzsVUuoHA4Jm6KunbsotaahvVikZs1JyTW6iJvbn \
  --clone HxBTMuB7cFBPVWVJjTi9iBF8MPd7mfY1QnrrWfLAySFd \
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