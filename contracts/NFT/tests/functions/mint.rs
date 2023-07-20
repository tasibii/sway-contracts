use crate::utils::{
    abi_calls::{mint, owner_of, balance_of},
    test_helpers::setup,
};
use fuels::{prelude::*, signers::Signer, types::{ Address, ContractId, Identity }};

mod success {

    use super::*;
    use crate::utils::{TransferEvent};    

    #[tokio::test]
    async fn self_mint() {
        
        let (_, bob, alice) = setup().await;
        
        // ================================ B O B ================================ //
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));

        assert_eq!(balance_of(&bob.contract, bob_identity.clone().unwrap()).await, 0);
        let bob_mint_response = mint(&bob.contract, bob_identity.clone().unwrap()).await;
        let bob_mint_log = bob_mint_response.get_logs_with_type::<TransferEvent>().unwrap();
        let bob_mint_event = bob_mint_log.get(0).unwrap();
        assert_eq!(
            *bob_mint_event,
            TransferEvent {
                from: None,
                to: bob_identity.clone(),
                token_id: 1,
            }
        );
        assert_eq!(owner_of(&bob.contract, 1).await, bob_identity.clone());
        assert_eq!(balance_of(&bob.contract, bob_identity.clone().unwrap()).await, 1);


        // ============================== A L I C E ============================== //
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));

        assert_eq!(balance_of(&bob.contract, alice_identity.clone().unwrap()).await, 0);
        let alice_mint_response = mint(&alice.contract, alice_identity.clone().unwrap()).await;
        let alice_mint_log = alice_mint_response.get_logs_with_type::<TransferEvent>().unwrap();
        let alice_mint_event = alice_mint_log.get(0).unwrap();
        assert_eq!(
            *alice_mint_event,
            TransferEvent {
                from: None,
                to: alice_identity.clone(),
                token_id: 2,
            }
        );
        assert_eq!(owner_of(&alice.contract, 2).await, alice_identity.clone());
        assert_eq!(balance_of(&alice.contract, alice_identity.clone().unwrap()).await, 1);
    }

    #[tokio::test]
    async fn mint_to() {
        let (_, bob, alice) = setup().await;

        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        let alice_identity = Some(Identity::Address(alice.wallet.address().into()));
        
        // ================================ B O B ================================ //
        assert_eq!(balance_of(&bob.contract, alice_identity.clone().unwrap()).await, 0);
        let bob_mint_response = mint(&bob.contract, alice_identity.clone().unwrap()).await;
        let bob_mint_log = bob_mint_response.get_logs_with_type::<TransferEvent>().unwrap();
        let bob_mint_event = bob_mint_log.get(0).unwrap();
        assert_eq!(
            *bob_mint_event,
            TransferEvent {
                from: None,
                to: alice_identity.clone(),
                token_id: 1,
            }
        );
        assert_eq!(owner_of(&bob.contract, 1).await, alice_identity.clone());
        assert_eq!(balance_of(&bob.contract, alice_identity.clone().unwrap()).await, 1);


        // ============================== A L I C E ============================== //
        assert_eq!(balance_of(&bob.contract, bob_identity.clone().unwrap()).await, 0);
        let alice_mint_response = mint(&alice.contract, bob_identity.clone().unwrap()).await;
        let alice_mint_log = alice_mint_response.get_logs_with_type::<TransferEvent>().unwrap();
        let alice_mint_event = alice_mint_log.get(0).unwrap();
        assert_eq!(
            *alice_mint_event,
            TransferEvent {
                from: None,
                to: bob_identity.clone(),
                token_id: 2,
            }
        );
        assert_eq!(owner_of(&alice.contract, 2).await, bob_identity.clone());
        assert_eq!(balance_of(&alice.contract, bob_identity.clone().unwrap()).await, 1);
    }
}

mod reverts {

    use super::*;
    
    #[tokio::test]
    #[should_panic(expected = "TransferToZeroIdentity")]
    async fn when_to_is_zero_identity() {
        let (_, bob, _) = setup().await;
        let zero_address = Address::from([0; 32]);
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));

        let zero_identity_address = Identity::Address(zero_address.into());
        mint(&bob.contract, zero_identity_address.clone()).await;

        let zero_contract_id = ContractId::from([0; 32]);
        let zero_identity_contact = Identity::ContractId(ContractId::from(zero_contract_id));
        mint(&bob.contract, zero_identity_contact.clone()).await;
    }
}