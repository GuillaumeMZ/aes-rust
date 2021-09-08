use super::aes_cipher;

type Aes256Key = [[u8; 16]; 14];

pub struct Aes256{
    keys: Aes256Key
}

impl Aes256{
    pub fn new(initial_key : &aes_cipher::DataBlock){

    }

    fn expand_key(&mut self){

    }
}

impl aes_cipher::AesCipher for Aes256{
    
}