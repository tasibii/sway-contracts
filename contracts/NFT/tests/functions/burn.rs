use crate::utils::{
    abi_calls::{ mint, burn, balance_of },
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;
    use crate::utils::{TransferEvent};

    #[tokio::test]
    async fn owner_burn() {
        let (_, bob, _) = setup().await;
        
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        mint(&bob.contract, bob_identity.clone().unwrap()).await;
        assert_eq!(balance_of(&bob.contract, bob_identity.clone().unwrap()).await, 1);

        let response = burn(&bob.contract, 1).await;
        let log = response.get_logs_with_type::<TransferEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            TransferEvent {
                from: bob_identity.clone(),
                to: None,
                token_id: 1,
            }
        );
        assert_eq!(balance_of(&bob.contract, bob_identity.clone().unwrap()).await, 0);
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn when_sender_is_not_owner_or_not_approved() {
        let (_, bob, alice) = setup().await;
        
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));

        mint(&bob.contract, bob_identity.clone().unwrap()).await;
        burn(&alice.contract, 1).await;
    }

    // #[tokio::test]
    // #[should_panic(expected = "DoesNotExist")]
    // async fn when_token_does_not_map_to_existing_token() {
    //     let (_, bob, _) = setup().await;
        
    //     let bob_identity = Some(Identity::Address(bob.wallet.address().into()));

    //     burn(&bob.contract, 1).await;
    // }
}