use crate::types::Meme;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, setup_alloc};
use std::collections::HashSet;

mod types;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SimpleMemeMuseum {
    museos: UnorderedMap<String, HashSet<u64>>,
    memes: UnorderedMap<u64, Meme>,
}

impl Default for SimpleMemeMuseum {
    fn default() -> Self {
        Self {
            museos: UnorderedMap::new(b"u".to_vec()),
            memes: UnorderedMap::new(b"e".to_vec()),
        }
    }
}

#[near_bindgen]
impl SimpleMemeMuseum {
    pub fn crear_meme(&mut self, titulo: String, datos: String, nombre_museo: String) {
        //Creamos el objeto del meme
        let meme = Meme::new(String::from(&titulo), String::from(&datos));

        //Lo guardamos en la lista general de memes
        self.memes.insert(&meme.id, &meme);

        //Buscamos si el museo existe, si no, lo creamos
        let museo = self.museos.get(&nombre_museo);

        //Si existe, agregamos el nuevo id de ese meme
        if museo.is_some() {
            museo.unwrap().insert(meme.id);
        }
        //Si no existe, creamos un nuevo museo, le agregamos el meme y lo guardamos.
        else {
            let mut nuevo_museo = HashSet::new();

            nuevo_museo.insert(meme.id);
            self.museos.insert(&nombre_museo, &nuevo_museo);
        }
    }
}
