use super::aes_cipher;

type Aes192Key = [[u8; 16]; 12]; 

pub struct Aes192{
    keys : Aes192Key
}

impl Aes192{
    pub fn new(initial_key : &aes_cipher::DataBlock){

    }

    fn expand_key(&mut self){

    }
}

impl aes_cipher::AesCipher for Aes192{
    
}