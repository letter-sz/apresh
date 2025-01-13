dfx canister create app && \
dfx canister create contract && \
dfx canister create system_api && \
dfx canister create internet_identity && \
source scripts/deploy_icrc_ledger.sh && \
(dfx build || dfx build) && \
dfx deploy