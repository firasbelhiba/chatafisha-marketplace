use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, collections::UnorderedMap};
use near_sdk::serde::{Serialize,Deserialize};

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct NFTMetadata {
    pub student_name: String,
    pub image: String,
    pub description: String,
    pub plastic_kg: u32,
    pub plastic_type: String,
    pub area_location: String,
}
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct NFT {
    pub owner_id: String,
    pub metadata: NFTMetadata,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Marketplace {
    pub nfts: UnorderedMap<String, NFT>,
    pub minter_nfts: UnorderedMap<String, Vec<String>>,
}

impl Default for Marketplace {
    fn default() -> Self {
        Self {
            nfts: UnorderedMap::new(b"nfts".to_vec()),
            minter_nfts: UnorderedMap::new(b"minter_nfts".to_vec()),
        }
    }
}

#[near_bindgen]
impl Marketplace {
    #[init]
    pub fn new() -> Self {
        Self {
            nfts: UnorderedMap::new(b"nfts".to_vec()),
            minter_nfts: UnorderedMap::new(b"minter_nfts".to_vec()),
        }
    }

    pub fn mint_nft(&mut self, metadata: NFTMetadata) {
        let owner_id = env::signer_account_id().to_string();
        let nft_id = env::random_seed().iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        let nft = NFT { owner_id: owner_id.clone(), metadata };
        self.nfts.insert(&nft_id, &nft);
    
        let mut minter_nfts = self.minter_nfts.get(&owner_id).unwrap_or_else(|| vec![]);
        minter_nfts.push(nft_id);
        self.minter_nfts.insert(&owner_id, &minter_nfts);
    }
    

    pub fn get_wallet_nfts(&self, owner_id: String) -> Option<Vec<NFT>> {
        self.minter_nfts.get(&owner_id).map(|nft_ids| {
            nft_ids.into_iter().filter_map(|nft_id| self.nfts.get(&nft_id)).collect()
        })
    }

    pub fn get_all_nfts(&self) -> Vec<(String, NFT)> {
        self.nfts.to_vec()
    }
}
