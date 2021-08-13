use super::*;

// traits can be compared to interfaces in e.g. java
pub trait Hashable {
    fn bytes (&self) -> Vec<u8>;
    
    fn hash (&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())

    }
}