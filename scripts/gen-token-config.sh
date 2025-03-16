#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status
set -o pipefail  # Pipe failure will be considered as command failure

MINT_AUTHORITY=$(solana-keygen pubkey)
echo Mint Authority: $MINT_AUTHORITY

# Function to create token, extract address, and dump account info
create_and_dump_token() {
  local decimals=$1
  local output_file=$2

  # Create token with optional decimals and capture the output
  if [ -n "$decimals" ]; then
    token_output=$(spl-token create-token --decimals $decimals --output json)
  else
    token_output=$(spl-token create-token --output json)
  fi

  # Extract the token address from the JSON output
  token_address=$(echo $token_output | jq -r '.commandOutput.address')

  # Check if the token address was successfully extracted
  if [ -z "$token_address" ]; then
    echo "Error: Unable to extract token address."
    exit 1
  fi

  # Print the token address for verification
  echo "Token Address: $token_address"

  # Dump the account information into a JSON file
  solana account $token_address --output json > $output_file

  # Check if the dump was successful
  if [ $? -eq 0 ]; then
    echo "Account data dumped successfully to $output_file"
  else
    echo "Error: Failed to dump account data to $output_file"
    exit 1
  fi
}
``
# # Create USDC token with 6 decimals
# create_and_dump_token 6 "./deps/configs/usdc.json"

# # Create BTC token (default decimals)
# create_and_dump_token 8 "./deps/configs/wbtc.json"

# Create BTC token (default decimals)
create_and_dump_token 9 "./deps/configs/mint9.json"
