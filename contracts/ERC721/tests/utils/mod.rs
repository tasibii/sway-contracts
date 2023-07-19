use fuels::{prelude::*, programs::call_response::FuelCallResponse, types::Identity, types::SizedAsciiString};

abigen!(Contract(
    name = "erc721",
    abi = "./contracts/ERC721/out/debug/erc721-abi.json"
));
// /Users/tasiby/Desktop/Personal/sway-contracts/contracts/ERC721/out/debug/erc721-abi.json
pub struct Metadata {
    pub contract: erc721,
    pub wallet: WalletUnlocked,
}

pub mod paths {
    pub const NFT_CONTRACT_BINARY_PATH: &str = "./out/debug/erc721.bin";
    pub const NFT_CONTRACT_STORAGE_PATH: &str = "./out/debug/erc721-storage_slots.json";
}

pub mod abi_calls {
    use super::*;

    pub async fn owner(contract: &erc721) -> Option<Identity> {
        contract.methods().owner().call().await.unwrap().value
    }

    pub async fn approve(
        to: Identity,
        contract: &erc721,
        token_id: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .approve(to.clone(), token_id)
            .call()
            .await
            .unwrap()
    }

    pub async fn get_approved(
        contract: &erc721,
        token_id: u64,
    ) -> Identity {
        contract
            .methods()
            .get_approved(token_id)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn balance_of(contract: &erc721, owner: Identity) -> u64 {
        contract
            .methods()
            .balance_of(owner.clone())
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn constructor(
        contract: &erc721,
        name: SizedAsciiString<13>,
        symbol: SizedAsciiString<3>,
        base_uri: SizedAsciiString<19>,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .constructor(name, symbol, base_uri)
            .call()
            .await
            .unwrap()
    }

    pub async fn name(
        contract: &erc721,
    ) -> SizedAsciiString<13> {
        contract
            .methods()
            .name()
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn symbol(
        contract: &erc721,
    ) -> SizedAsciiString<3> {
        contract
            .methods()
            .symbol()
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn base_uri(
        contract: &erc721,
    ) -> SizedAsciiString<19> {
        contract
            .methods()
            .base_uri()
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn set_approval_for_all(
        contract: &erc721,
        operator: Identity,
        approved: bool,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .set_approval_for_all(operator, approved)
            .call()
            .await
            .unwrap()
    }

    pub async fn mint(
        contract: &erc721,
        to: Identity,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .mint(to)
            .call()
            .await
            .unwrap()
    }

    pub async fn transfer_from(
        contract: &erc721,
        from: Identity,
        to: Identity,
        token_id: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .transfer_from(from, to, token_id)
            .call()
            .await
            .unwrap()
    }

    pub async fn transfer_ownership(
        contract: &erc721,
        new_owner: Option<Identity>,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .transfer_ownership(new_owner)
            .call()
            .await
            .unwrap()
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
            contract: erc721::new(nft_id.clone(), wallet1.clone()),
            wallet: wallet1,
        };

        let owner1 = Metadata {
            contract: erc721::new(nft_id.clone(), wallet2.clone()),
            wallet: wallet2,
        };

        let owner2 = Metadata {
            contract: erc721::new(nft_id, wallet3.clone()),
            wallet: wallet3,
        };

        (deploy_wallet, owner1, owner2)
    }
}
