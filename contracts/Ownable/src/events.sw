library;


pub struct OwnershipRenounced {
    previous_owner: Identity,
}

pub struct OwnershipSet {
    new_owner: Identity,
}

pub struct OwnershipTransfered {
    previous_owner: Identity,
    new_owner: Identity,
}