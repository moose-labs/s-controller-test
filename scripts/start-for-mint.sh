#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status
set -o pipefail  # Pipe failure will be considered as command failure

# delete ledger directory
rm -rf ./test-ledger

# Start solana-test-validator with custom programs and token accounts
solana-test-validator --reset --ledger ./test-ledger \
  --url mainnet-beta \
  \
  --account 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq ./deps/configs/pre-fund.json \
  \
  --account jupSoLaHXQiZZTSfEWMTRRgpnyFm8f6sZdosWBjx93v ./deps/configs/mint9.json \
  --account BPSoLzmLQn47EP5aa7jmFngRL8KC3TWAeAwXwZD8ip3P ./deps/configs/mint9.json \
  --account Dso1bDeDjCQxTrWHqUUi63oBvV7Mdm6WaobLbQ7gnPQ ./deps/configs/mint9.json \
  --account mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So ./deps/configs/mint9.json \
  --account 5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm ./deps/configs/mint9.json \
  --account J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn ./deps/configs/mint9.json \
  --account he1iusmfkpAdwvxLNGV8Y1iSbj4rUy6yMhEA3fotn9A ./deps/configs/mint9.json


