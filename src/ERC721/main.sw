contract;

mod errors;
mod events;
mod interface;

use ::errors::ERC721Error;
use ::events::{Transfer, Approval, ApprovalForAll, OwnershipTransfered};
use ::interface::ERC721;
use std::{
    auth::msg_sender,
    identity::Identity,
    logging::log,
    option::Option,
    result::Result,
    revert::require,
    storage::StorageMap,
};

storage {
    initialized: bool = false,
    access_control: bool = false,
    owner: Option<Identity> = Option::None,
    balances: StorageMap<Identity, u64> = StorageMap{},
    owners: StorageMap<u64, Identity> = StorageMap{},
    token_approvals: StorageMap<u64, Identity> = StorageMap{},
    operator_approvals: StorageMap<(Identity, Identity), bool> = StorageMap{},
    total_supply: u64 = 0,
    base_uri: str[64] = "                                                                ",
    name: str[64] = "                                                                ",
    symbol: str[32] = "                                ",
}

impl ERC721 for Contract {
    #[storage(read, write)] 
    fn constructor(name: str[64], symbol: str[32], base_uri: str[64]) {
        require(storage.initialized == false, ERC721Error::CannotReinitialize);
        let owner = Option::Some(msg_sender().unwrap());
        storage.access_control = true;
        storage.owner = owner;
        storage.name = name;
        storage.symbol = symbol;
        storage.base_uri = base_uri;
    }

    #[storage(read)]
    fn owner() -> Identity {
        require(storage.initialized == true, ERC721Error::NotInitialized);
        storage.owner.unwrap()
    }

    #[storage(read)]
    fn balance_of(owner: Identity) -> u64 {
        storage.balances.get(owner).unwrap()
    }

    #[storage(read)]
    fn owner_of(token_id: u64) -> Identity {
        storage.owners.get(token_id).unwrap()
    }

    #[storage(read)]
    fn get_approved(token_id: u64) -> Identity {
        storage.token_approvals.get(token_id).unwrap()
    }

    #[storage(read)]
    fn symbol() -> str[32] {
        storage.symbol
    }

    #[storage(read)]
    fn name() -> str[64] {
        storage.name
    }

    #[storage(read)]
    fn base_uri() -> str[64] {
        storage.base_uri
    }

    #[storage(read)]
    fn total_supply() -> u64 {
        storage.total_supply
    }

    #[storage(read, write)]
    fn approve(to: Identity, token_id: u64) {
        let owner = storage.owners.get(token_id);
        require(owner.is_some(), ERC721Error::NotMintYet);
        require(owner.unwrap() != to, ERC721Error::InvalidOperator);
        require(owner.unwrap() == msg_sender().unwrap() || storage.operator_approvals.get((owner.unwrap(), msg_sender().unwrap())).unwrap(), ERC721Error::Unauthorized);

        storage.token_approvals.insert(token_id, to);
    }

    #[storage(write)]
    fn set_approval_for_all(operator: Identity, approved: bool) {
        require(msg_sender().unwrap() != operator, ERC721Error::InvalidOperator);
        storage.operator_approvals.insert((msg_sender().unwrap(), operator), approved);
    }

    #[storage(read, write)]
    fn mint(to: Identity) {
        require(storage.initialized == true, ERC721Error::NotInitialized);
        require(!storage.access_control || msg_sender().unwrap() == storage.owner.unwrap(), ERC721Error::Unauthorized);
        
        let token_id = storage.total_supply + 1;
        storage.owners.insert(token_id, to);
        storage.balances.insert(to, storage.balances.get(to).unwrap() + 1);
        storage.total_supply += 1;

        let zero_bit: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
        let zero_address: Address = Address::from(zero_bit);
        log(Transfer {
            from: Identity::Address(zero_address),
            to: to,
            token_id: token_id,
        })
    }

    #[storage(read, write)]
    fn transfer_from(from: Identity, to: Identity, token_id: u64) {
        let owner = storage.owners.get(token_id);
        require(owner.is_some(), ERC721Error::NotMintYet);
        require(storage.token_approvals.get(token_id).unwrap() == msg_sender().unwrap() || owner.unwrap() == msg_sender().unwrap(), ERC721Error::Unauthorized);

        storage.owners.insert(token_id, to);
        storage.balances.insert(from, storage.balances.get(from).unwrap() - 1);
        storage.balances.insert(to, storage.balances.get(to).unwrap() + 1);

        log(Transfer {
            from: from,
            to: to,
            token_id: token_id,
        })
    }

    #[storage(read, write)]
    fn transfer_ownership(new_owner: Identity) {
        let owner = storage.owner.unwrap();
        require(owner == msg_sender().unwrap(), ERC721Error::Unauthorized);
        storage.owner = Option::Some(new_owner);

        log(OwnershipTransfered {
            owner: owner,
            new_owner: new_owner,
        })
    }
}