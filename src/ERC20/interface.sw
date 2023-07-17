library;

use std::{
    identity::Identity,
    option::Option,
};

abi ERC721 {
    #[storage(read)]
    fn owner() -> Identity;

    #[storage(read)]
    fn balance_of(owner: Identity) -> u64;

    #[storage(read)]
    fn owner_of(token_id: u64) -> Identity;

    #[storage(read)]
    fn get_approved(token_id: u64) -> Identity;

    #[storage(read)]
    fn symbol() -> str[32];

    #[storage(read)]
    fn name() -> str[64];

    #[storage(read)]
    fn base_uri() -> str[64];

    #[storage(read)]
    fn total_supply() -> u64;

    #[storage(read, write)]
    fn constructor(name: str[64], symbol: str[32], base_uri: str[64]);

    #[storage(read, write)]
    fn approve(to: Identity, token_id: u64);

    #[storage(write)]
    fn set_approval_for_all(operator: Identity, approved: bool);

    #[storage(read, write)]
    fn mint(to: Identity);

    #[storage(read, write)]
    fn transfer_from(from: Identity, to: Identity, token_id: u64);

    #[storage(read, write)]
    fn transfer_ownership(new_owner: Identity);
}
