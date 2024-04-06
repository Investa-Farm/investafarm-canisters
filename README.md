# INVESTA FARM CANISTERS
Welcome to Investa Farm Canisters. This repository contains the different backend logic for Investa Farm Site 

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

The architecture is divided into different sections: 

1. Canister one: responsible for entity management and ordering of items from supply agri business 
2. Canister two: responsible for approval of the different entities that have registered
3. Canister three: this will be the payments smart contract

### 1. Canister One: ``src/canister_one``
It contains the following logic:
1. Entity Management: Registration, Updating and Deletion of the different entities in the system (i.e farms, Investors, Farms Agri Business, Supply Agri Business). The logic for this is located in the ``src/entitymanagement.rs`` 
2. Ordering of items from supply agri business: Farmers will be able to order the different items that are being sold by the different supply agri businesses that have registered in the system. 

### 2. Canister Two: ``src/canister_two``
Responsible for approval of the different entities that have registered once they go through the KYC 

### 3. Canister Three: ``src/canister_three``
Responsible for payments 

## Running the code: 
1. Step 1: Starting the local replica: 
```
dfx start --clean --background
```

2. Step 2: Generating the candid file: 
```
For canister 1: 
./canister_one_did.sh && dfx generate canister_one

For canister 2:
./canister_two_did.sh && dfx generate canister_two

For canister 3:
./canister_three_did.sh && dfx generate canister_three
```

3. Step 3: Deploying the canister locally 
```
For canister 1: 
dfx deploy canister_one 

For canister 2: 
dfx deploy canister_two 

For canister 3: 
dfx deploy canister_three
```
