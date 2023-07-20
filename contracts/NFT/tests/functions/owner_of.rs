use crate::utils::{
    abi_calls::{owner_of, mint},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_owner_of() {
        let (_, bob, _) = setup().await;
        let bob_identity = Some(Identity::Address(bob.wallet.address().into()));
        mint(&bob.contract, bob_identity.clone().unwrap()).await;
        assert_eq!(owner_of(&bob.contract, 1).await, bob_identity);
    }
}