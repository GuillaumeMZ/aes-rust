
pub type DataBlock = [u8; 16];

pub trait AesCipher {
    fn encrypt(&self, block : &mut DataBlock);
    fn decrypt(&self, block : &mut DataBlock);
}

//Fonctions