use super::common;

type Aes192Key = [[u8; 16]; 12]; 

pub struct Aes192{
    keys : Aes192Key
}

impl Aes192{
    pub fn new(initial_key : &common::DataBlock){

    }

    fn expand_key(&mut self){

    }
}

impl common::AesCipher for Aes192{
    fn encrypt(&self, block: &mut common::DataBlock) {

    }

    fn decrypt(&self, block: &mut common::DataBlock) {

    }
}