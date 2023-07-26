contract;

mod errors;
mod events;
mod interface;

use std::auth::msg_sender;

use ::interface::NFT;
use ::errors::TokenError;

const ZERO_B256: b256 = b256::min();
const ZERO_ADDRESS: Address = Address::from(ZERO_B256);
const ZERO_CONTRACT_ID: ContractId = ContractId::from(ZERO_B256);

storage {
    token_id: u64 = 0,
    owner_of: StorageMap<u64, Identity> = StorageMap {},
    balance_of: StorageMap<Identity, u64> = StorageMap {},
    approvals: StorageMap<u64, Identity> = StorageMap {},
    is_approved_for_all: StorageMap<(Identity, Identity), bool> = StorageMap {},
}

#[storage(read)]
fn is_approved_or_owner(
    owner: Identity,
    spender: Identity,
    token_id: u64,
) -> bool {
    if owner == spender {
        return true;
    }

    if let Option::Some(approved_identity) = storage.approvals.get(token_id) {
        if approved_identity == spender {
            return true;
        }
    }

    return storage.is_approved_for_all.get((owner, spender)).unwrap_or(false);
}

fn is_zero_identity(id: Identity) -> bool {
    match id {
        Identity::Address(addr) => addr == ZERO_ADDRESS,
        Identity::ContractId(contract_id) => contract_id == ZERO_CONTRACT_ID,
    }
}

impl NFT for Contract {
    //Read methods
    #[storage(read)]
    fn owner_of(token_id: u64) -> Option<Identity> {
        storage.owner_of.get(token_id)
    }
    #[storage(read)]
    fn balance_of(owner: Identity) -> u64 {
        storage.balance_of.get(owner).unwrap_or(0)
    }
    #[storage(read)]
    fn approvals(token_id: u64) -> Option<Identity> {
        storage.approvals.get(token_id)
    }
    #[storage(read)]
    fn is_approved_for_all(owner: Identity, operator: Identity) -> bool {
        storage.is_approved_for_all.get((owner, operator)).unwrap_or(false)
    }

    //Write methods
    #[storage(read, write)]
    fn mint(to: Identity) {
        require(!is_zero_identity(to), TokenError::TransferToZeroIdentity);

        storage.token_id += 1;
        let token_id = storage.token_id;
        
        storage.owner_of.insert(token_id, to);
        let bal = storage.balance_of.get(to).unwrap_or(0);
        storage.balance_of.insert(to, bal + 1);

        log(events::TransferEvent {
            token_id,
            from: Option::None,
            to: Option::Some(to)
        });
    }
    #[storage(read, write)]
    fn burn(token_id: u64) {
        let owner = storage.owner_of.get(token_id).unwrap();
        let sender = msg_sender().unwrap();
        require(owner == sender, TokenError::NotOwner);
        require(storage.owner_of.remove(token_id), TokenError::DoesNotExist);

        storage.approvals.remove(token_id);
        let bal = storage.balance_of.get(sender).unwrap_or(0);
        storage.balance_of.insert(sender, bal - 1);

        log(events::TransferEvent {
            token_id,
            from: Option::Some(sender),
            to: Option::None,
        });
    }
    #[storage(write)]
    fn set_approval_for_all(operator: Identity, approved: bool) {
        let sender = msg_sender().unwrap();
        
        storage.is_approved_for_all.insert((sender, operator), approved);

        log(events::ApprovalForAllEvent {
            owner: sender,
            operator,
            approved,
        });
    }
    #[storage(read, write)]
    fn approve(spender: Identity, token_id: u64) {
        let owner = storage.owner_of.get(token_id).unwrap();
        let sender = msg_sender().unwrap();
        // TODO: find way to revert TokenNotExist
        require(owner == sender || storage.is_approved_for_all.get((owner, sender)).unwrap_or(false), TokenError::NotAuthorized);

        storage.approvals.insert(token_id, spender);

        log(events::ApprovalEvent {
            owner,
            spender,
            token_id,
        });
    }
    #[storage(read, write)]
    fn transfer_from(from: Identity, to: Identity, token_id: u64) {
        let owner = storage.owner_of.get(token_id).unwrap();
        let sender = msg_sender().unwrap();

        require(owner == from, TokenError::NotOwner);
        require(is_approved_or_owner(owner, sender, token_id), TokenError::NotAuthorized);
        require(!is_zero_identity(to), TokenError::TransferToZeroIdentity);

        storage.balance_of.insert(from, storage.balance_of.get(from).unwrap() - 1);
        storage.balance_of.insert(to, storage.balance_of.get(to).unwrap_or(0) + 1);
        storage.owner_of.insert(token_id, to);
        storage.approvals.remove(token_id);

        log(events::TransferEvent {
            token_id,
            from: Option::Some(from),
            to: Option::Some(to),
        });
    }
}