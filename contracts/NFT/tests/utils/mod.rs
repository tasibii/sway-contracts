use fuels::{prelude::*, programs::call_response::FuelCallResponse, types::Identity};

abigen!(Contract(
    name = "nft",
    abi = "./contracts/NFT/out/debug/nft-abi.json"
));

pub struct Metadata {
    pub contract: nft,
    pub wallet: WalletUnlocked,
}

pub mod paths {
    pub const NFT_CONTRACT_BINARY_PATH: &str = "./out/debug/nft.bin";
    pub const NFT_CONTRACT_STORAGE_PATH: &str = "./out/debug/nft-storage_slots.json";
}

pub mod abi_calls {
    use super::*;

    // =============================== R E A D =============================== //
    pub async fn owner_of(contract: &nft, token_id: u64) -> Option<Identity> {
        contract.methods().owner_of(token_id).call().await.unwrap().value
    }
    
    pub async fn balance_of(contract: &nft, owner: Identity) -> u64 {
        contract.methods().balance_of(owner).call().await.unwrap().value
    }

    pub async fn approvals(contract: &nft, token_id: u64) -> Option<Identity> {
        contract.methods().approvals(token_id).call().await.unwrap().value
    }

    pub async fn is_approved_for_all(contract: &nft, owner: Identity, operator: Identity) -> bool {
        contract.methods().is_approved_for_all(owner, operator).call().await.unwrap().value
    }

    // ============================== W R I T E ============================== //
    pub async fn mint(contract: &nft, to: Identity) -> u64 {
        contract.methods().mint(to).call().await.unwrap().value
    }
    
    pub async fn burn(contract: &nft, token_id: u64) -> FuelCallResponse<()> {
        contract.methods().burn(token_id).call().await.unwrap()
    }

    pub async fn set_approval_for_all(contract: &nft, operator: Identity, approved: bool) -> FuelCallResponse<()> {
        contract.methods().set_approval_for_all(operator, approved).call().await.unwrap()
    }

    pub async fn approve(contract: &nft, spender: Identity, token_id: u64) -> FuelCallResponse<()> {
        contract.methods().approve(spender, token_id).call().await.unwrap()
    }

    pub async fn transfer_from(contract: &nft, from: Identity, to: Identity, token_id: u64) -> FuelCallResponse<()> {
        contract.methods().transfer_from(from, to, token_id).call().await.unwrap()
    }
}

pub mod test_helpers {

    use super::*;
    use paths::{NFT_CONTRACT_BINARY_PATH, NFT_CONTRACT_STORAGE_PATH};

    pub async fn setup() -> (Metadata, Metadata, Metadata) {
        let num_wallets = 3;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;

        let mut wallets = launch_custom_provider_and_get_wallets(
            WalletsConfig::new(
                Some(num_wallets),
                Some(coins_per_wallet),
                Some(amount_per_coin),
            ),
            None,
            None,
        )
        .await;

        // Get the wallets from that provider
        let wallet1 = wallets.pop().unwrap();
        let wallet2 = wallets.pop().unwrap();
        let wallet3 = wallets.pop().unwrap();

        let nft_id = Contract::deploy(
            NFT_CONTRACT_BINARY_PATH,
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(NFT_CONTRACT_STORAGE_PATH.to_string())),
        )
        .await
        .unwrap();

        let deploy_wallet = Metadata {
            contract: nft::new(nft_id.clone(), wallet1.clone()),
            wallet: wallet1,
        };

        let owner1 = Metadata {
            contract: nft::new(nft_id.clone(), wallet2.clone()),
            wallet: wallet2,
        };

        let owner2 = Metadata {
            contract: nft::new(nft_id, wallet3.clone()),
            wallet: wallet3,
        };

        (deploy_wallet, owner1, owner2)
    }
}
