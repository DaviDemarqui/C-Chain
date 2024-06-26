use chrono::Utc;
use serde::{Deserialize, Serialize};

pub struct App {
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub timestamp: i64,
    pub previous_hash: String,
    pub data: String,
    pub nonce: u64
}

fn main() {
    impl App {
        fn new() -> Self {
            Self { blocks: vec![] }
        }

        fn genesis(&mut self) {
            let genesis_block = Block {
                id: 0,
                timestamp: Utc::now().timestamp(),
                previous_hash: String::from("genesis"),
                data: String::from("genesis!"),
                nonce: 2836,
                hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
            };

            self.blocks.push(genesis_block);
        }
    }
}
