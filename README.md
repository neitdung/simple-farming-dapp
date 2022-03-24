# simple-farming-dapp
NEAR simple farming app includes contracts and frontend (client) app

## Initialization
near create-account contract.simple-farming.testnet --masterAccount simple-farming.testnet
./build.sh
near deploy --wasmFile res/out.wasm --accountId contract.simple-farming.testnet
near call contract.simple-farming.testnet new '{"owner_id": "contract.simple-farming.testnet"}' --accountId contract.simple-farming.testnet

near deploy --wasmFile res/fungible_token.wasm --accountId ft.neitgnud.testnet
near call ft.neitgnud.testnet new '{"owner_id": "ft.neitgnud.testnet", "total_supply": "10000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Neit Token", "symbol": "NEIT", "decimals": 8 }}' --accountId ft.neitgnud.testnet

near call ft.neitgnud.testnet storage_deposit --accountId neitgnud.testnet --deposit 0.125
near call ft.neitgnud.testnet ft_transfer '{"receiver_id": "neitgnud.testnet", "amount": "10000000"}' --accountId ft.neitgnud.testnet --depositYocto 1

## Deposit
near call contract.simple-farming.testnet storage_deposit --accountId dungbss.testnet --deposit 1

## Create farm
near call contract.simple-farming.testnet ft_deposit '{"ft_account": "ft.neitgnud.testnet"}' --accountId neitgnud.testnet --deposit 0.25
near call ft.neitgnud.testnet ft_transfer_call '{ "receiver_id": "contract.simple-farming.testnet", "amount": "5000000", "msg": "create_farm"}' --accountId neitgnud.testnet --depositYocto 1 --gas 100000000000000

near call contract.simple-farming.testnet create_farm '{"start_at": "0", "reward_per_session": "2", "session_interval": "10"}' --deposit 0.5 --accountId neitgnud.testnet

## View farms
near view contract.simple-farming.testnet list_farms '{"from_index": 0, "limit": 10}'
near view contract.simple-farming.testnet list_seeds 
near view ft.neitgnud.testnet ft_balance_of '{"account_id": "contract.simple-farming.testnet"}'
near view ft.neitgnud.testnet storage_balance_of '{"account_id": "contract.simple-farming.testnet"}'

## Stake farm
near call contract.simple-farming.testnet stake '{"farm_id": "ft.neitgnud.testnet#2"}' --accountId dungbss.testnet --deposit 2

## Claim farm
near call contract.simple-farming.testnet claim_reward_by_farm '{"farm_id": "ft.neitgnud.testnet#2"}' --accountId dungbss.testnet --gas 100000000000000

## Withdraw
near call contract.simple-farming.testnet withdraw '{"farm_id": "ft.neitgnud.testnet#3", "amount": "1000000000000000000000000"}' --accountId dungbss.testnet

## Demo step call
near deploy --wasmFile res/fungible_token.wasm --accountId ft.dungss.testnet
near call ft.dungss.testnet new '{"owner_id": "ft.dungss.testnet", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Test2 Token", "symbol": "TEST2", "decimals": 8 }}' --accountId ft.dungss.testnet
near call ft.dungss.testnet storage_deposit --accountId dungtestbss.testnet --deposit 0.125
near call ft.dungss.testnet ft_transfer '{"receiver_id": "dungtestbss.testnet", "amount": "300000000"}' --accountId ft.dungss.testnet --depositYocto 1

near call contract.simple-farming.testnet ft_deposit '{"ft_account": "ft.dungss.testnet"}' --accountId dungtestbss.testnet --deposit 0.25
near call ft.dungss.testnet ft_transfer_call '{ "receiver_id": "contract.simple-farming.testnet", "amount": "9000000", "msg": "create_farm"}' --accountId dungtestbss.testnet --depositYocto 1 --gas 100000000000000
near call contract.simple-farming.testnet create_farm '{"start_at": "0", "reward_per_session": "5", "session_interval": "4"}' --deposit 0.5 --accountId dungtestbss.testnet

near call contract.simple-farming.testnet storage_deposit --accountId dungtestbss.testnet --deposit 1
near call ft.dungss.testnet storage_deposit --accountId dungtestbss.testnet --deposit 0.125
near call contract.simple-farming.testnet stake '{"farm_id": "ft.dungss.testnet#0"}' --accountId dungtestbss.testnet --deposit 2
near call contract.simple-farming.testnet stake '{"farm_id": "ft.dungss.testnet#0"}' --accountId dungtestbss.testnet --deposit 4

near call contract.simple-farming.testnet claim_reward_by_farm '{"farm_id": "ft.dungss.testnet#0"}' --accountId dungtestbss.testnet --gas 100000000000000

near call contract.simple-farming.testnet withdraw '{"farm_id": "ft.dungss.testnet#0", "amount": "100000000000000000000000"}' --accountId dungtestbss.testnet --depositYocto 1 --gas 100000000000000
