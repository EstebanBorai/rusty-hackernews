use sha2::{Digest, Sha256};

pub fn hash(raw: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(raw.as_bytes());

    format!("{:X}", hasher.finalize())
}

#[cfg(test)]
mod test {
    use super::hash;

    #[test]
    fn hashes_a_string() {
        let hashed = hash("Hello World");

        assert_eq!(
            hashed,
            "A591A6D40BF420404A011733CFB7B190D62C65BF0BCDA32B57B277D9AD9F146E"
        );
    }
}
