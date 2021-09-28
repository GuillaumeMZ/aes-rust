use super::common;

type Aes128Key = [[u8; 16]; 11];

pub struct Aes128{
    keys : Aes128Key
}

fn fill_next_key(current_key: &[u8; 16], new_key: &mut [u8; 16], index: usize) {
    new_key[0]  = current_key[7];
    new_key[4]  = current_key[11];
    new_key[8]  = current_key[15];
    new_key[12] = current_key[3];

    for i in (0..13).step_by(4) {
        new_key[i] = common::SUBTABLE[new_key[i] as usize];
    }

    new_key[0] ^= common::RCON[index];

    for i in (0..13).step_by(4){
        new_key[i] ^= current_key[i];
    }

    for i in 0..16{
        if i % 4 != 0{
            new_key[i] = new_key[i-1] ^ current_key[i];   
        }
    }
}

impl Aes128{
    pub fn new(initial_key : &common::DataBlock) -> Self{
        let mut result = Aes128{
            keys: [[0; 16]; 11]
        };

        result.keys[0] = *initial_key;
        result.expand_key();
        result
    }

    fn expand_key(&self){
        for i in 1..11 {
            
        }
    }

    fn apply_key(&self, data: &mut common::DataBlock, key_index: usize) {
        let requested_key = &self.keys[key_index];

        for i in 0..16 {
            data[i] ^= requested_key[i];
        }
    }
}

impl common::AesCipher for Aes128{
    fn encrypt(&self, block: &mut common::DataBlock) {
        self.apply_key(block, 0);

        for i in 1..10 {
            common::sub_bytes(block);
            common::shift_rows(block);
            //mix_columns
            self.apply_key(block, i);
        }

        common::sub_bytes(block);
        common::shift_rows(block);
        self.apply_key(block, 10);
    }

    fn decrypt(&self, block: &mut common::DataBlock) {
        
    }
}