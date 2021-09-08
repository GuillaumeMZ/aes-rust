mod aes;

type Aes256Key = [[u8; 16]; 14];

pub struct Aes256{
    keys: Aes256Key;
}

impl Aes256{
    pub fn new(initial_key : &Block){

    }

    fn expand_key(&mut self){

    }
}

impl AesCipher for Aes256{
    
}