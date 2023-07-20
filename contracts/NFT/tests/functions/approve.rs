use crate::utils::{
    abi_calls::{approve, approvals, mint, owner_of, balance_of, transfer_from, set_approval_for_all, is_approved_for_all},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;
    use crate::utils::{TransferEvent, ApprovalEvent, ApprovalForAllEvent};    

    #[tokio::test]
    async fn approves() {
        
        let (deployer, bob, alice) = setup().await;
        
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));
        
        let minted_token = mint(&bob.contract, bob_identity.clone().unwrap()).await;
        assert_eq!(minted_token, 1);

        let response = approve(&bob.contract, alice_identity.clone().unwrap(), minted_token).await;
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

        assert_eq!(approvals(&bob.contract, minted_token).await, alice_identity.clone());
    }

    #[tokio::test]
    async fn approves_for_all() {
        let (deployer, bob, alice) = setup().await;
        
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));

        let first_nft = mint(&bob.contract, bob_identity.clone().unwrap()).await;
        let second_nft = mint(&bob.contract, bob_identity.clone().unwrap()).await;
        let third_nft = mint(&bob.contract, bob_identity.clone().unwrap()).await;
        assert_eq!(third_nft, 3);

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

    #[tokio::test]
    #[should_panic(expected = "DoesNotExist")]
    async fn when_token_does_not_map_to_existing_token() {
        let (deployer, bob, alice) = setup().await;
        
        // let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));
        approve(&alice.contract, alice_identity.clone().unwrap(), 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotAuthorized")]
    async fn when_sender_is_not_owner() {
        let (deployer, bob, alice) = setup().await;
        
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));
        
        let minted_token = mint(&bob.contract, bob_identity.clone().unwrap()).await;
        assert_eq!(minted_token, 1);

        approve(&alice.contract, alice_identity.clone().unwrap(), minted_token).await;
    }
}