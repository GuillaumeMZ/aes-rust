mod aes;

fn main() {
    let key = aes::Aes128::new(&[0u8; 16]); 
}
