#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status
set -o pipefail  # Pipe failure will be considered as command failure

spl-token create-account EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
spl-token create-account 3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh

spl-token mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v 500000000
spl-token mint 3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh 500000000


## User account
solana airdrop 10000 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq
solana airdrop 10000 BG4gEQXRWBVJmcE56Jc6UoL8nUfujvRo6r4dJP8wSLsW

spl-token create-account EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v --owner 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq --fee-payer ./deps/configs/local-auth.json
spl-token create-account 3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh --owner 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq --fee-payer ./deps/configs/local-auth.json

spl-token mint --recipient-owner 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v 10000
spl-token mint --recipient-owner 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq 3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh 10000

