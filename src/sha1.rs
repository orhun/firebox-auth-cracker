use sha1::{Digest, Sha1};

/// Returns the SHA1 hash of the given data.
pub fn sha1_hash(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sha1_rand() {
        assert_eq!(
            sha1_hash(b"hello world"),
            "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"
        );
    }
}
