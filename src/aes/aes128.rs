mod aes;

type Aes128Key = [[u8; 16]; 10];

pub struct Aes128{
    keys : Aes128Key;
}

impl Aes128{
    pub fn new(initial_key : &Block) -> Self{
        let mut result = Aes128{
            keys: [[0; 16]; 10];
        }

        result
    }

    fn expand_key(&mut Aes128Key){

    }
}

impl AesCipher for Aes128{
    
}