mod aes;

type Aes192Key = [[u8; 16]; 12]; 

pub struct Aes192{
    keys : Aes192Key;
}

impl Aes192{
    pub fn new(initial_key : &Block){

    }

    fn expand_key(&mut self){

    }
}

impl AesCipher for Aes192{
    
}