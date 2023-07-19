use crate::utils::{
    abi_calls::{owner, constructor, transfer_ownership},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::{Identity, SizedAsciiString}};

mod success {

    use super::*;
    use crate::utils::OwnershipTransfered;

    #[tokio::test]
    async fn gets_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        assert_eq!(owner(&owner1.contract).await, None);

        let _owner = Some(Identity::Address(deploy_wallet.wallet.address().into()));
        let name = SizedAsciiString::<13>::new("suy va lo the".to_string()).unwrap();
        let symbol = SizedAsciiString::<3>::new("suy".to_string()).unwrap();
        let base_uri = SizedAsciiString::<19>::new("ipfs://metadata/suy".to_string()).unwrap();

        constructor(&deploy_wallet.contract, name, symbol, base_uri).await;

        assert_eq!(owner(&deploy_wallet.contract).await, _owner.clone());
    }

    #[tokio::test]
    async fn gets_owner_after_transfer_ownership() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        assert_eq!(owner(&owner1.contract).await, None);

        let _owner = Some(Identity::Address(deploy_wallet.wallet.address().into()));
        let name = SizedAsciiString::<13>::new("suy va lo the".to_string()).unwrap();
        let symbol = SizedAsciiString::<3>::new("suy".to_string()).unwrap();
        let base_uri = SizedAsciiString::<19>::new("ipfs://metadata/suy".to_string()).unwrap();

        constructor(&deploy_wallet.contract, name, symbol, base_uri).await;

        assert_eq!(owner(&deploy_wallet.contract).await, _owner.clone());

        let _new_owner = Some(Identity::Address(owner1.wallet.address().into()));
        let response = transfer_ownership(&deploy_wallet.contract, _new_owner.clone()).await;
        let log = response.get_logs_with_type::<OwnershipTransfered>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(owner(&deploy_wallet.contract).await, _new_owner.clone());
        assert_eq!(
            *event,
            OwnershipTransfered {
                owner: _owner.unwrap().clone(),
                new_owner: _new_owner.unwrap().clone(),
            }
        );

    }
}