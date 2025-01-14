#!/bin/bash

used=$(dfx identity whoami)

dfx identity use anonymous

dfx canister call icrc1_ledger_canister icrc1_transfer "(record {                  
  amount = 200_000_000;
  to = record {
    owner = principal \"$1\";  
  };  
})"

dfx identity use $used