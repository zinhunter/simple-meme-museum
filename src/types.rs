use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Meme {
    pub id: u64,
    pub creado_por: String,
    pub titulo: String,
    pub datos: String,
    pub votos: u32,
}

impl Default for Meme {
    fn default() -> Self {
        Meme {
            id: 0,
            creado_por: String::from(""),
            titulo: String::from(""),
            datos: String::from(""),
            votos: 0,
        }
    }
}

impl Meme {
    pub fn new(titulo: String, datos: String) -> Self {
        Self {
            id: env::block_index(),
            creado_por: env::signer_account_id(),
            titulo,
            datos,
            votos: 0,
        }
    }
}
