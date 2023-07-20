use crate::utils::{
    abi_calls::{approve, set_approval_for_all, mint, transfer_from, owner_of},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::{Address, ContractId, Identity}};

mod success {

    use super::*;
    use crate::utils::{TransferEvent};    

    #[tokio::test]
    async fn owner_transfer() {
        let (_, bob, alice) = setup().await;
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));
        
        mint(&bob.contract, bob_identity.clone().unwrap()).await;
        assert_eq!(owner_of(&bob.contract, 1).await, bob_identity.clone());

        let response = transfer_from(&bob.contract, bob_identity.clone().unwrap(), alice_identity.clone().unwrap(), 1).await;
        let log = response.get_logs_with_type::<TransferEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            TransferEvent {
                from: bob_identity.clone(),
                to: alice_identity.clone(),
                token_id: 1,
            }
        );

        assert_eq!(owner_of(&bob.contract, 1).await, alice_identity.clone());
    }

    #[tokio::test]
    async fn approved_transfer() {
        let (_, bob, alice) = setup().await;
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));
        
        mint(&bob.contract, bob_identity.clone().unwrap()).await;
        assert_eq!(owner_of(&bob.contract, 1).await, bob_identity.clone());
        approve(&bob.contract, alice_identity.clone().unwrap(), 1).await;

        let response = transfer_from(&alice.contract, bob_identity.clone().unwrap(), alice_identity.clone().unwrap(), 1).await;
        let log = response.get_logs_with_type::<TransferEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            TransferEvent {
                from: bob_identity.clone(),
                to: alice_identity.clone(),
                token_id: 1,
            }
        );

        assert_eq!(owner_of(&bob.contract, 1).await, alice_identity.clone());
    }

    #[tokio::test]
    async fn approved_all_transfer() {
        let (_, bob, alice) = setup().await;
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));
        
        mint(&bob.contract, bob_identity.clone().unwrap()).await;
        assert_eq!(owner_of(&bob.contract, 1).await, bob_identity.clone());
        set_approval_for_all(&bob.contract, alice_identity.clone().unwrap(), true).await;

        let response = transfer_from(&alice.contract, bob_identity.clone().unwrap(), alice_identity.clone().unwrap(), 1).await;
        let log = response.get_logs_with_type::<TransferEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            TransferEvent {
                from: bob_identity.clone(),
                to: alice_identity.clone(),
                token_id: 1,
            }
        );

        assert_eq!(owner_of(&bob.contract, 1).await, alice_identity.clone());
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn when_from_is_not_owner() {
        let (peter, bob, alice) = setup().await;
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        let peter_identity = Some(Identity::Address(peter.wallet.address().into()));
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));

        mint(&bob.contract, bob_identity.clone().unwrap()).await;
        transfer_from(&bob.contract, peter_identity.clone().unwrap(), alice_identity.clone().unwrap(), 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotAuthorized")]
    async fn when_sender_is_not_owner_or_not_approved() {
        let (_, bob, alice) = setup().await;
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));

        mint(&bob.contract, bob_identity.clone().unwrap()).await;
        transfer_from(&alice.contract, bob_identity.clone().unwrap(), alice_identity.clone().unwrap(), 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "TransferToZeroIdentity")]
    async fn when_to_is_zero_identity() {
        let (_, bob, _) = setup().await;
        let zero_address = Address::from([0; 32]);
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));

        mint(&bob.contract, bob_identity.clone().unwrap()).await;

        let zero_identity_address = Identity::Address(zero_address.into());
        transfer_from(&bob.contract, bob_identity.clone().unwrap(), zero_identity_address.clone(), 1).await;

        let zero_contract_id = ContractId::from([0; 32]);
        let zero_identity_contact = Identity::ContractId(ContractId::from(zero_contract_id));
        transfer_from(&bob.contract, bob_identity.clone().unwrap(), zero_identity_contact.clone(), 1).await;
    }
}