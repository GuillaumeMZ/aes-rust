mod tests;
mod aes;
use rand;

pub fn generate_random_block() -> aes::Block {
    let mut result = [0u8; 16];
    for i in (0..16){
        result[i] = rand::random::<u8>();
    }

    result
}