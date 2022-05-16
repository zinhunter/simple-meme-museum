use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc, Promise};

setup_alloc!();

// **************** CLASES ****************

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Meme {
    pub id: u64,
    pub creado_por: String,
    pub titulo: String,
    pub museo: String,
    pub url: String,
    pub donaciones: u128,
}

impl Default for Meme {
    fn default() -> Self {
        Meme {
            id: 0,
            creado_por: String::from(""),
            titulo: String::from(""),
            museo: String::from(""),
            url: String::from(""),
            donaciones: 0,
        }
    }
}

impl Meme {
    pub fn new(titulo: String, url: String, museo: String) -> Self {
        Self {
            id: env::block_index(),
            creado_por: env::signer_account_id(),
            titulo,
            museo,
            url,
            donaciones: 0,
        }
    }
}

// **************** INICIALIZACIÓN ****************

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SimpleMemeMuseum {
    //Guardamos solo los ID para evitar tener que editar en ambos lugares cuando se modifique un meme.
    museos: UnorderedMap<String, Vec<u64>>,
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

// **************** MÉTODOS ****************

#[near_bindgen]
impl SimpleMemeMuseum {
    pub fn crear_meme(&mut self, titulo: String, url: String, nombre_museo: String) {
        //Creamos el objeto del meme
        let meme = Meme::new(
            String::from(&titulo),
            String::from(&url),
            String::from(&nombre_museo),
        );

        //Lo guardamos en la lista general de memes
        self.memes.insert(&meme.id, &meme);

        //Buscamos si el museo existe, si no, lo creamos
        let museo = self.museos.get(&nombre_museo);

        //Si existe, agregamos el nuevo id de ese meme
        if museo.is_some() {
            let mut m = museo.unwrap();
            m.push(meme.id);

            self.museos.insert(&nombre_museo, &m);
        }
        //Si no existe, creamos un nuevo museo, le agregamos el meme y lo guardamos.
        else {
            let mut nuevo_museo = Vec::new();

            nuevo_museo.push(meme.id);
            self.museos.insert(&nombre_museo, &nuevo_museo);
        }

        env::log(
            format!(
                "Nuevo meme añadido con éxito. Museo: {}, Id Meme: {}",
                &nombre_museo, meme.id
            )
            .as_bytes(),
        )
    }

    //Si encuentra el meme lo muestra
    pub fn obtener_meme(&self, id: u64) -> Option<Meme> {
        self.memes.get(&id)
    }

    pub fn obtener_lista_memes(&self) -> Vec<(u64, Meme)> {
        self.memes.to_vec()
    }

    pub fn obtener_lista_museos(&self) -> Vec<String> {
        self.museos.keys_as_vector().to_vec()
    }

    //Regresamos un Vector con los memes que tiene ese museo.
    pub fn obtener_memes_museo(&self, nombre_museo: String) -> Vec<Meme> {
        let museo = self.museos.get(&nombre_museo);

        if museo.is_some() {
            let mut lista_memes = Vec::new();

            for meme in &museo.unwrap() {
                let m = self.memes.get(meme);

                if m.is_some() {
                    lista_memes.push(m.unwrap());
                }
            }
            lista_memes
        } else {
            Vec::new()
        }
    }

    #[payable]
    pub fn donar_a_meme(&mut self, id: u64) -> bool {
        assert!(
            env::attached_deposit() > 0,
            "Debes de agregar NEAR para hacer una donación."
        );

        //Buscamos el meme
        match self.memes.get(&id) {
            Some(mut meme) => {
                //Si existe, guardamos la donación en el registro

                meme.donaciones += env::attached_deposit();
                self.memes.insert(&meme.id, &meme);

                //Y le transferimos al creador del meme lo que le donaron
                Promise::new(String::from(&meme.creado_por)).transfer(env::attached_deposit());

                true
            }
            None => false,
        }
    }
}
