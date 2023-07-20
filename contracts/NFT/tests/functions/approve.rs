use crate::utils::{
    abi_calls::{approve, approvals, mint, set_approval_for_all, is_approved_for_all},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;
    use crate::utils::{ApprovalEvent, ApprovalForAllEvent};    

    #[tokio::test]
    async fn approves() {
        
        let (_, bob, alice) = setup().await;
        
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));
        
        mint(&bob.contract, bob_identity.clone().unwrap()).await;

        let response = approve(&bob.contract, alice_identity.clone().unwrap(), 1).await;
        let log = response.get_logs_with_type::<ApprovalEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ApprovalEvent {
                owner: bob_identity.clone().unwrap(),
                spender: alice_identity.clone().unwrap(),
                token_id: 1,
            }
        );

        assert_eq!(approvals(&bob.contract, 1).await, alice_identity.clone());
    }

    #[tokio::test]
    async fn approves_for_all() {
        let (_, bob, alice) = setup().await;
        
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));

        mint(&bob.contract, bob_identity.clone().unwrap()).await;
        mint(&bob.contract, bob_identity.clone().unwrap()).await;
        mint(&bob.contract, bob_identity.clone().unwrap()).await;

        let response = set_approval_for_all(&bob.contract, alice_identity.clone().unwrap(), true).await;
        let log = response.get_logs_with_type::<ApprovalForAllEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            ApprovalForAllEvent {
                owner: bob_identity.clone().unwrap(),
                operator: alice_identity.clone().unwrap(),
                approved: true,
            }
        );

        assert_eq!(is_approved_for_all(&bob.contract, bob_identity.clone().unwrap(), alice_identity.clone().unwrap()).await, true);
    }
}

mod reverts {

    use super::*;

    // #[tokio::test]
    // #[should_panic(expected = "DoesNotExist")]
    // async fn when_token_does_not_map_to_existing_token() {
    //     let (_, _, alice) = setup().await;

    //     let alice_identity = Some(Identity::Address(alice.wallet.address().into()));

    //     approve(&alice.contract, alice_identity.clone().unwrap(), 0).await;
    // }

    #[tokio::test]
    #[should_panic(expected = "NotAuthorized")]
    async fn when_sender_is_not_owner() {
        let (_, bob, alice) = setup().await;
        
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));
        
        mint(&bob.contract, bob_identity.clone().unwrap()).await;

        approve(&alice.contract, alice_identity.clone().unwrap(), 1).await;
    }
}