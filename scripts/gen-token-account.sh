#!/bin/bash
set -e  # Exit immediately if a command exits with a non-zero status

create_and_dump_token_account() {
    # Set the owner address (used for both creating the token account and receiving minted tokens)
    local OWNER=$1
    # Token mint (e.g., jupsol)
    local TOKEN_MINT=$2
    # Number of tokens to mint
    local TOKEN_AMOUNT=$3
    # Output file for token account data
    local NAME=$4

    echo "Creating associated token account for mint: $TOKEN_MINT..."
    spl-token create-account "$TOKEN_MINT" --owner "$OWNER" --fee-payer ./deps/configs/user1.json

    echo "Minting $TOKEN_AMOUNT tokens from mint: $TOKEN_MINT to owner: $OWNER..."
    spl-token mint --recipient-owner "$OWNER" "$TOKEN_MINT" "$TOKEN_AMOUNT"

    # Wait a moment for the account creation to settle
    sleep 2

    # Get the token account address using spl-token accounts command and jq.
    # This command assumes the output is in JSON format and that there is only one account for this mint.
    TOKEN_ACCOUNT=$(spl-token accounts --owner "$OWNER" --output json | jq -r '.accounts[] | select(.mint=="'"$TOKEN_MINT"'") | .address')

    if [ -z "$TOKEN_ACCOUNT" ]; then
        echo "Error: Could not find token account for mint $TOKEN_MINT"
        exit 1
    fi

    echo "Found token account: $TOKEN_ACCOUNT"
    echo "Exporting token account data to $NAME..."
    solana account "$TOKEN_ACCOUNT" --output json -o "$NAME"
}

# Token accounts to prepare:
# jupsol                     jupSoLaHXQiZZTSfEWMTRRgpnyFm8f6sZdosWBjx93v
# bpsol                      BPSoLzmLQn47EP5aa7jmFngRL8KC3TWAeAwXwZD8ip3P
# dsol                       Dso1bDeDjCQxTrWHqUUi63oBvV7Mdm6WaobLbQ7gnPQ
# msol                       mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So
# INF                        5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm
# jitosol                    J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn
# hsol                       he1iusmfkpAdwvxLNGV8Y1iSbj4rUy6yMhEA3fotn9A

create_and_dump_token_account 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq jupSoLaHXQiZZTSfEWMTRRgpnyFm8f6sZdosWBjx93v 10001 ./deps/configs/token-account-jupsol.json
create_and_dump_token_account 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq BPSoLzmLQn47EP5aa7jmFngRL8KC3TWAeAwXwZD8ip3P 10002 ./deps/configs/token-account-bpsol.json
create_and_dump_token_account 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq Dso1bDeDjCQxTrWHqUUi63oBvV7Mdm6WaobLbQ7gnPQ 10003 ./deps/configs/token-account-dsol.json
create_and_dump_token_account 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So 10004 ./deps/configs/token-account-msol.json
create_and_dump_token_account 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq 5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm 10005 ./deps/configs/token-account-INF.json
create_and_dump_token_account 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn 10006 ./deps/configs/token-account-jitosol.json
create_and_dump_token_account 54GvzEwe25N55wJ8zQZ4YTFjFAnDVP6fj4ZrvVhAyafq he1iusmfkpAdwvxLNGV8Y1iSbj4rUy6yMhEA3fotn9A 10007 ./deps/configs/token-account-hsol.json

echo "Token account creation and export complete."