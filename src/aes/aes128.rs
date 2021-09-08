use super::aes_cipher;

type Aes128Key = [[u8; 16]; 10];

pub struct Aes128{
    keys : Aes128Key
}

impl Aes128{
    pub fn new(initial_key : &aes_cipher::DataBlock) -> Self{
        let mut result = Aes128{
            keys: [[0; 16]; 10]
        };

        result
    }

    fn expand_key(&mut key_array: Aes128Key){

    }
}

impl aes_cipher::AesCipher for Aes128{
    
}