library;

abi NFT {
    // Read methods
    #[storage(read)]
    fn owner_of(token_id: u64) -> Option<Identity>;
    #[storage(read)]
    fn balance_of(owner: Identity) -> u64;
    #[storage(read)]
    fn approvals(token_id: u64) -> Option<Identity>;
    #[storage(read)]
    fn is_approved_for_all(owner: Identity, operator: Identity) -> bool;

    // Write methods
    #[storage(read, write)]
    fn mint(to: Identity) -> u64;
    #[storage(read, write)]
    fn burn(token_id: u64);
    #[storage(write)]
    fn set_approval_for_all(operator: Identity, approved: bool);
    #[storage(read, write)]
    fn approve(spender: Identity, token_id: u64);
    #[storage(read, write)]
    fn transfer_from(from: Identity, to: Identity, token_id: u64);
}

