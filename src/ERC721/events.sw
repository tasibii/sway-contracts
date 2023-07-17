library;

use std::{
    identity::Identity,
    option::Option,
};

pub struct Transfer {
    from: Identity,
    to: Identity,
    token_id: u64,
}

pub struct Approval {
    owner: Identity,
    approved: Identity,
    token_id: u64,
}

pub struct ApprovalForAll {
    owner: Identity,
    operator: Identity,
    approved: bool,
}

pub struct OwnershipTransfered {
    owner: Identity,
    new_owner: Identity,
}
