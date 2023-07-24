library;

pub mod data_structures;
pub mod errors;
pub mod events;

use data_structures::State;
use errors::AccessError;
use events::{OwnershipRenounced, OwnershipSet, OwnershipTransfered};
use std::{auth::msg_sender, hash::sha256, storage::storage_api::{read, write}};

pub struct  Ownership {
    owner: State,
}

impl Ownership {
    pub fn uninitialized() -> Self {
        Self {
            owner: State::Uninitialized,
        }
    }

    pub fn initialized(identity: Identity) -> Self {
        Self {
            owner: State::Initialized(identity),
        }
    }

    pub fn revoked() -> Self {
        Self {
            owner: State::Revoked,
        }
    }
}

impl StorageKey<Ownership> {
    #[storage(read)]
    pub fn owner(self) -> State {
        self.read().owner
    }
}

impl StorageKey<Ownership> {
    #[storage(read)]
    pub fn only_owner(self) {
        require(self.owner() == State::Initialized(msg_sender().unwrap()), AccessError::NotOwner);
    }
}

impl StorageKey<Ownership> {
    #[storage(read, write)]
    pub fn renounce_ownership(self) {
        self.only_owner();

        self.write(Ownership::revoked());

        log(OwnershipRenounced {
            previous_owner: msg_sender().unwrap(),
        });
    }

    #[storage(read, write)]
    pub fn set_ownership(self, new_owner: Identity) {
        require(self.owner() == State::Uninitialized, AccessError::CannotReinitialize);

        self.write(Ownership::initialized(new_owner));

        log(OwnershipSet {new_owner});
    }

    #[storage(read, write)]
    pub fn transfer_ownership (self, new_owner: Identity) {
        self.only_owner();
        self.write(Ownership::initialized(new_owner));

        log(OwnershipTransfered {
            previous_owner: msg_sender().unwrap(),
            new_owner,
        });
    }
}