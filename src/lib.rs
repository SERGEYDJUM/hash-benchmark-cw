use gost94::{Digest, Gost94CryptoPro};
use md5;
use sha256;

#[inline]
pub fn digest_sha256(data: &[u8]) -> String {
    sha256::digest(data)
}

#[inline]
pub fn digest_md5(data: &[u8]) -> String {
    hex::encode(md5::compute(data).0)
}

#[inline]
pub fn digest_cryptopro(data: &[u8]) -> String {
    let mut hasher = Gost94CryptoPro::new();
    hasher.update(data);
    hex::encode(&hasher.finalize()[..])
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read;

    #[test]
    fn hash_calculation() {
        assert_eq!(
            digest_cryptopro("Hello World!".as_bytes()),
            "636a32a952ecb9e8529ea759ecff1c33623945e5d868352a7df5f240ea747ded"
        );

        assert_eq!(
            digest_sha256("Hello World!".as_bytes()),
            "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069"
        );

        assert_eq!(
            digest_md5("Hello World!".as_bytes()),
            "ed076287532e86365e841e92bfc50d8c"
        );
    }

    #[test]
    fn md5_collision() {
        let a = read("colliding_md5/collide_one").expect("Could not read first colliding file");
        let b = read("colliding_md5/collide_two").expect("Could not read second colliding file");
        assert_ne!(a, b);
        assert_eq!(digest_md5(&a), digest_md5(&b));
    }
}
