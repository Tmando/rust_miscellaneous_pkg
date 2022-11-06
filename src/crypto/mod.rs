//! Collection of crypto algorithms
pub mod crypto {
    use bytes::Bytes;

    /// return a sha224 hash of a string
    pub fn get_sha224(input: Bytes) -> String {
        let hash = openssl::sha::sha224(&input);
        return hex::encode(hash);
    }
    /// return a sha256 hash of a string
    pub fn get_sha256(input: Bytes) -> String {
        let hash = openssl::sha::sha256(&input);
        return hex::encode(hash);
    }

    /// return a sha384 hash of a string
    pub fn get_sha384(input: Bytes) -> String {
        let hash = openssl::sha::sha384(&input);
        return hex::encode(hash);
    }

    /// return a sha512 hash of a string
    pub fn get_sha512(input: Bytes) -> String {
        let hash = openssl::sha::sha512(&input);
        return hex::encode(hash);
    }

    /// return a sha1 hash of a string
    pub fn get_sha1(input: Bytes) -> String {
        let hash = openssl::sha::sha1(&input);
        return hex::encode(hash);
    }
}

mod test {
    #[test]
    fn test_get_sha224() {
        assert_eq!(
            super::crypto::get_sha224(bytes::Bytes::from("Hello World")),
            "c4890faffdb0105d991a461e668e276685401b02eab1ef4372795047".to_string()
        );
    }
    #[test]
    fn test_get_sha256() {
        assert_eq!(
            super::crypto::get_sha256(bytes::Bytes::from("Hello World")),
            "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e".to_string()
        );
    }

    #[test]
    fn test_get_sha384() {
        assert_eq!(
            super::crypto::get_sha384(bytes::Bytes::from("Hello World")),
            "99514329186b2f6ae4a1329e7ee6c610a729636335174ac6b740f9028396fcc803d0e93863a7c3d90f86beee782f4f3f".to_string()
        );
    }

    #[test]
    fn test_get_sha512() {
        assert_eq!(
            super::crypto::get_sha512(bytes::Bytes::from("Hello World")),
            "2c74fd17edafd80e8447b0d46741ee243b7eb74dd2149a0ab1b9246fb30382f27e853d8585719e0e67cbda0daa8f51671064615d645ae27acb15bfb1447f459b".to_string()
        );
    }

    #[test]
    fn test_get_sha1() {
        assert_eq!(
            super::crypto::get_sha1(bytes::Bytes::from("Hello World")),
            "0a4d55a8d778e5022fab701977c5d840bbc486d0".to_string()
        );
    }
}