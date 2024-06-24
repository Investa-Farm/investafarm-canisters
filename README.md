# INVESTA FARM CANISTERS
Welcome to Investa Farm Canisters. This repository contains the different backend logic for Investa Farm Site 

## Mainnet Deployement Link:
URL for 1st mainnet deployement: ``https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=4p7ji-qaaaa-aaaal-qjabq-cai`` (Canister One)

## Getting started: 
Clone the repository
```
git clone https://github.com/Investa-Farm/investafarm-canisters.git
```

Ensure you have the following dependencies installed in your environment
```
1. Node js 18 
2. dfx 
3. Rust 
```

You can check out the following guides on how to set up your environment and install the required dependencies: 
1. Guide 1: [Link](https://internetcomputer.org/docs/current/developer-docs/getting-started/install/) 
2. Guide 2: [Link](https://docs.google.com/document/d/1OW3oT8F9pumYg3hmybrHFB8T0VpDwDgRVE5PfVkHFJI/edit?usp=sharing)

## Canister Architecture: 
A detailed explanation of the canister's architecture can be found over [here](https://docs.google.com/document/d/1EGoq2N2qiWPbeFbTOVr1LSLiaspwg4ROMYodqY8TkfU/edit?usp=sharing)

A detailed explanation of the user flow can be found ove [here](https://docs.google.com/document/d/115tZG5oz6jwoKw-9cYmaolUvrzHONh1ILVRjYcSQYx8/edit?usp=sharing)

The architecture is divided into two sections: 

1. Canister one: responsible for entity management and ordering of items from supply agri business 
2. Payments smart contract: payments smart contract responsible for payments in USDC and USDT (Stablecoins) [Link](https://github.com/Investa-Farm/investafarm-payments)

## Running the code: 
1. Step 1: Starting the local replica: 
```
dfx start --clean --background
```

2. Step 2: Generating the candid file: 
```
For canister 1: 
./canister_one_did.sh && dfx generate canister_one


3. Step 3: Deploying the canister locally 
```
For canister 1: 
dfx deploy canister_one 

```