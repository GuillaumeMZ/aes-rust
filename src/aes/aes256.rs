use super::common;

type Aes256Key = [[u8; 16]; 14];

pub struct Aes256{
    keys: Aes256Key
}

impl Aes256{
    pub fn new(initial_key : &common::DataBlock){

    }

    fn expand_key(&mut self){

    }
}

impl common::AesCipher for Aes256{
    fn encrypt(&self, block: &mut common::DataBlock) {
       
    }

    fn decrypt(&self, block: &mut common::DataBlock) {

    }
}