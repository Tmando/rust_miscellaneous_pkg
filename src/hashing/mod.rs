//! This a module with a collection of different hashing methods
//! This is a wrapper for [tiny_keccak](https://docs.rs/tiny-keccak/latest/tiny_keccak/)

#[cfg(feature = "hashing")]
pub mod hashing {
    use tiny_keccak::Hasher;
    use tiny_keccak::IntoXof;
    use tiny_keccak::Xof;
    /// This is a wrapper for [cs_shake_v128](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.CShake.html)
    pub fn cs_shake_v128(name: &[u8], custom_string: &[u8],input:&[u8])->[u8;32]{
        let mut output = [0; 32];
        let mut cshake_v128 = tiny_keccak::CShake::v128(name, custom_string);
        cshake_v128.update(input);
        cshake_v128.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [cs_shake_v256](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.CShake.html)
    pub fn cs_shake_v256(name: &[u8], custom_string: &[u8],input:&[u8])->[u8;32]{
        let mut output = [0; 32];
        let mut cshake_v256 = tiny_keccak::CShake::v256(name, custom_string);
        cshake_v256.update(input);
        cshake_v256.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [kangaroo_twelve](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.KangarooTwelve.html)
    pub fn kangaroo_twelve(custom_string: &[u8],input:&[u8])->[u8;32]{
        let mut output = [0; 32];
        let mut kangaroo_twelve_hasher = tiny_keccak::KangarooTwelve::new(custom_string);
        kangaroo_twelve_hasher.update(input);
        kangaroo_twelve_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [kangaroo_twelve_xof](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.KangarooTwelveXof.html)
    pub fn kangaroo_twelve_xof(custom_string: &[u8],input: &[u8])->[u8; 64]{
        let mut output = [0; 64];
        let mut kangaroo_twelve_hasher = tiny_keccak::KangarooTwelve::new(custom_string);
        kangaroo_twelve_hasher.update(input);
        let mut kangaroo_twelve_hasher_xof = kangaroo_twelve_hasher.into_xof();
        kangaroo_twelve_hasher_xof.squeeze(&mut output[..32]);
        kangaroo_twelve_hasher_xof.squeeze(&mut output[32..]);
        return output;
    }
    /// This is a wrapper for [keccak_v224](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.Keccak.html)
    pub fn keccak_v224(input: &[u8])->[u8; 32]{
        let mut output = [0u8; 32];
        let mut keccak_v224_hasher = tiny_keccak::Keccak::v224();
        keccak_v224_hasher.update(input);
        keccak_v224_hasher.finalize(&mut output);
        return output

    }
    /// This is a wrapper for [keccak_v256](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.Keccak.html)
    pub fn keccak_v256(input: &[u8])->[u8; 32]{
        let mut output = [0u8; 32];
        let mut keccak_v256_hasher = tiny_keccak::Keccak::v256();
        keccak_v256_hasher.update(input);
        keccak_v256_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [keccak_v384](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.Keccak.html)
    pub fn keccak_v384(input: &[u8])->[u8; 32]{
        let mut output = [0u8; 32];
        let mut keccak_v384_hasher = tiny_keccak::Keccak::v384();
        keccak_v384_hasher.update(input);
        keccak_v384_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [keccak_v512](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.Keccak.html)
    pub fn keccak_v512(input: &[u8])->[u8; 32]{
        let mut output = [0u8; 32];
        let mut keccak_v512_hasher = tiny_keccak::Keccak::v512();
        keccak_v512_hasher.update(input);
        keccak_v512_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [kmac_v128](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.Kmac.html)
    pub fn kmac_v128(key: &[u8], custom_string: &[u8],input:&[u8])->[u8; 32]{
        let mut output = [0u8; 32];
        let mut kmac_v128_hasher = tiny_keccak::Kmac::v128(key, custom_string);
        kmac_v128_hasher.update(input);
        kmac_v128_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [kmac_v256](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.Kmac.html)
    pub fn kmac_v256(key: &[u8], custom_string: &[u8],input:&[u8])->[u8; 32]{
        let mut output = [0u8; 32];
        let mut kmac_v256_hasher = tiny_keccak::Kmac::v256(key, custom_string);
        kmac_v256_hasher.update(input);
        kmac_v256_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [kmac_v256_xof](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.KmacXof.html)
    pub fn kmac_v256_xof(key: &[u8],input:&[u8], custom_string: &[u8])->[u8; 64]{
        let mut output = [0u8; 64];
        let mut kmac_v256_hasher = tiny_keccak::Kmac::v256(key, custom_string);
        kmac_v256_hasher.update(input);
        let mut kmac_v256_hasher_xof = kmac_v256_hasher.into_xof();
        kmac_v256_hasher_xof.squeeze(&mut output[..32]);
        kmac_v256_hasher_xof.squeeze(&mut output[32..]);
        return output;
    }
    /// This is a wrapper for [kmac_v128_xof](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.KmacXof.html)
    pub fn kmac_v128_xof(key: &[u8],input: &[u8], custom_string: &[u8])->[u8;64]{
        let mut output = [0u8; 64];
        let mut kmac_v128_hasher = tiny_keccak::Kmac::v128(key, custom_string);
        kmac_v128_hasher.update(input);
        let mut kmac_v128_hasher_xof = kmac_v128_hasher.into_xof();
        kmac_v128_hasher_xof.squeeze(&mut output[..32]);
        kmac_v128_hasher_xof.squeeze(&mut output[32..]);
        return output;
    }
    /// This is a wrapper for [parallel_hash_v128](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.ParallelHash.html)
    pub fn parallel_hash_v128(custom_string: &[u8],input: &[u8], block_size: usize)->[u8; 32]{
        let mut output = [0u8; 32];
        let mut parallel_hash_v128_hasher = tiny_keccak::ParallelHash::v128(custom_string, block_size);
        parallel_hash_v128_hasher.update(input);
        parallel_hash_v128_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [parallel_hash_v256](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.ParallelHash.html)
    pub fn parallel_hash_v256(custom_string: &[u8],input: &[u8], block_size: usize)->[u8; 32]{
        let mut output = [0u8; 32];
        let mut parallel_hash_v256_hasher = tiny_keccak::ParallelHash::v256(custom_string, block_size);
        parallel_hash_v256_hasher.update(input);
        parallel_hash_v256_hasher.finalize(&mut output);
        return output;
    }
    
    /// This is a wrapper for [parallel_hash_xof_v128](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.ParallelHashXof.html)
    pub fn parallel_hash_xof_v128(custom_string: &[u8],input:&[u8], block_size: usize)->[u8; 64]{
        let mut output = [0u8; 64];
        let mut parallel_hash_v128_hasher = tiny_keccak::ParallelHash::v128(custom_string, block_size);
        parallel_hash_v128_hasher.update(input);
        let mut xof = parallel_hash_v128_hasher.into_xof();
        xof.squeeze(&mut output[..32]);
        xof.squeeze(&mut output[32..]);
        return output;
    }
    /// This is a wrapper for [parallel_hash_xof_v256](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.ParallelHashXof.html)
    pub fn parallel_hash_xof_v256(custom_string: &[u8], input:&[u8], block_size: usize)->[u8; 64]{
        let mut output = [0u8; 64];
        let mut parallel_hash_v256_hasher = tiny_keccak::ParallelHash::v256(custom_string, block_size);
        parallel_hash_v256_hasher.update(input);
        let mut xof = parallel_hash_v256_hasher.into_xof();
        xof.squeeze(&mut output[..32]);
        xof.squeeze(&mut output[32..]);
        return output;
    }
    /// This is a wrapper for [sha3_v224](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.Sha3.html)
    pub fn sha3_v224(input: &[u8])->[u8;32]{
        let mut output = [0u8; 32];
        let mut sha_v224_hasher = tiny_keccak::Sha3::v224();
        sha_v224_hasher.update(input);
        sha_v224_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [sha3_v256](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.Sha3.html)
    pub fn sha3_v256(input: &[u8])->[u8;32]{
        let mut output = [0u8; 32];
        let mut sha_v256_hasher = tiny_keccak::Sha3::v256();
        sha_v256_hasher.update(input);
        sha_v256_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [sha3_v384](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.Sha3.html)
    pub fn sha3_v384(input: &[u8])->[u8; 32]{
        let mut output = [0u8; 32];
        let mut sha_v384_hasher = tiny_keccak::Sha3::v384();
        sha_v384_hasher.update(input);
        sha_v384_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [sha3_v512](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.Sha3.html)
    pub fn sha3_v512(input: &[u8])->[u8; 32]{
        let mut output = [0u8; 32];
        let mut sha_v512_hasher = tiny_keccak::Sha3::v512();
        sha_v512_hasher.update(input);
        sha_v512_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [shake_v128](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.Shake.html)
    pub fn shake_v128(input: &[u8])->[u8; 32]{
        let mut output = [0u8; 32];
        let mut sha_v128_hasher = tiny_keccak::Shake::v128();
        sha_v128_hasher.update(input);
        sha_v128_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [shake_v256](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.Shake.html)
    pub fn shake_v256(input: &[u8])->[u8; 32]{
        let mut output = [0u8; 32];
        let mut sha_v256_hasher = tiny_keccak::Shake::v256();
        sha_v256_hasher.update(input);
        sha_v256_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [tuple_hash_v128](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.TupleHash.html)
    pub fn tuple_hash_v128(custom_string: &[u8],input:&[u8])->[u8; 32]{
        let mut output = [0u8; 32];
        let mut tuple_hash_v128_hasher = tiny_keccak::TupleHash::v128(custom_string);
        tuple_hash_v128_hasher.update(input);
        tuple_hash_v128_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [tuple_hash_v256](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.TupleHash.html)
    pub fn tuple_hash_v256(custom_string: &[u8],input:&[u8])->[u8; 32]{
        let mut output = [0u8; 32];
        let mut tuple_hash_v256_hasher = tiny_keccak::TupleHash::v256(custom_string);
        tuple_hash_v256_hasher.update(input);
        tuple_hash_v256_hasher.finalize(&mut output);
        return output;
    }
    /// This is a wrapper for [tuple_hash_v128_xof](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.TupleHashXof.html)
    pub fn tuple_hash_v128_xof(custom_string: &[u8],input:&[u8])->[u8; 64]{
        let mut output = [0u8; 64];
        let mut tuple_hash_v128_hasher = tiny_keccak::TupleHash::v128(custom_string);
        tuple_hash_v128_hasher.update(input);
        let mut tuple_hash_v128_hasher_xof = tuple_hash_v128_hasher.into_xof();
        tuple_hash_v128_hasher_xof.squeeze(&mut output[..32]);
        tuple_hash_v128_hasher_xof.squeeze(&mut output[32..]);
        return output;
    }
    /// This is a wrapper for [tuple_hash_v256_xof](https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.TupleHashXof.html)
    pub fn tuple_hash_v256_xof(custom_string: &[u8],input:&[u8])->[u8; 64]{
        let mut output = [0u8; 64];
        let mut tuple_hash_v256_hasher = tiny_keccak::TupleHash::v256(custom_string);
        tuple_hash_v256_hasher.update(input);
        let mut tuple_hash_v256_hasher_xof = tuple_hash_v256_hasher.into_xof();
        tuple_hash_v256_hasher_xof.squeeze(&mut output[..32]);
        tuple_hash_v256_hasher_xof.squeeze(&mut output[32..]);
        return output;
    }
}

#[cfg(feature = "hashing")]
mod test {
    #[test]
    fn test_cs_shake_v256(){
        assert_eq!(hex::encode(super::hashing::cs_shake_v256(b"",b"",b"Hallo Welt")), "f17c4c09e18c298f11f475b0bb20d37309d5490c57423962fea7a9a56b0e39d0".to_string());
    }
    #[test]
    fn test_kangaroo_twelve(){
        assert_eq!(hex::encode(super::hashing::kangaroo_twelve(b"",b"Hallo Welt")), "a62498cd99af15d84bb32c4473b041a8a0a4b104da3612b8c13b69a2a4b3cf9c".to_string());
    }
    #[test]
    fn test_kangaroo_twelve_xof(){
        assert_eq!(hex::encode(super::hashing::kangaroo_twelve_xof(b"",b"Hallo Welt")), "a62498cd99af15d84bb32c4473b041a8a0a4b104da3612b8c13b69a2a4b3cf9c7b32dfedd6423b8e52d8468784f4429f6bb2602bb760059d1d5e91df4ae9e2f5".to_string());
    }

    #[test]
    fn test_keccak_v224(){
        assert_eq!(hex::encode(super::hashing::keccak_v224(b"Hallo Welt")), "4f979515a23781bf8c079aef785e1212a026df4bb20d88d3914434d8dc5646a6".to_string());
    }
    #[test]
    fn test_keccak_v256(){
        assert_eq!(hex::encode(super::hashing::keccak_v256(b"Hallo Welt")), "8c0f56949121b5c8d9e2496db9060e671137cecf0f4b178d351f15599ebee78d".to_string());
    }

    #[test]
    fn test_keccak_v384(){
        assert_eq!(hex::encode(super::hashing::keccak_v384(b"Hallo Welt")), "a32182feeadd445dcebd1b1a44edf4d05356a5e69353a1ee36ba23ddbd27146f".to_string());
    }

    #[test]
    fn test_keccak_v512(){
        assert_eq!(hex::encode(super::hashing::keccak_v512(b"Hallo Welt")), "3aad7fd5a8112272a3e0c91a9adaf09cf34c309f46f66ca63355a565d5b88f61".to_string());
    }

    #[test]
    fn test_kmac_v128(){
        assert_eq!(hex::encode(super::hashing::kmac_v128(b"Rust", b"",b"Hello world")), "abb4ec71ccbdaa4e4ef90842a52c21e8ed62ec1226f96ecb59c0320433205ffc".to_string());
    }
    #[test]
    fn test_kmac_v256(){
        assert_eq!(hex::encode(super::hashing::kmac_v256(b"Rust", b"",b"Hello world")), "e615661aacf10c14965a9152c609bf6a3d38eb600c94b4b790faa509675625c8".to_string());
    }

    #[test]
    fn test_kmac_v256_xof(){
        assert_eq!(hex::encode(super::hashing::kmac_v256_xof(b"Rust", b"",b"Hello world")), "7030b4505df62fcc904994f762dea369a92a2d28a2c5d523e5e0fa218e3f7bed95919145628882fb78a952689a02eb33b663e5fd97e3357ab5640225a3b93ff4".to_string());
    }
    #[test]
    fn test_kmac_v128_xof(){
        assert_eq!(hex::encode(super::hashing::kmac_v128_xof(b"Rust", b"",b"Hello world")), "02be891bb7a2cfa069669f51accfa710fed8eb2aae0f0397ee4a3a88c2cc261edf1938df736f03b3ed1e95de561575e0696a95fca26b58a566fc89e28bcb4107".to_string());
    }

    #[test]
    fn test_parallel_hash_v128(){
        assert_eq!(hex::encode(super::hashing::parallel_hash_v128(b"",b"Thomas Mandorfer",32 as usize)), "4b63672262a180c62196ee8ed82b801474d7d22ae174cbf877b1a515f1e0f129".to_string());
    }

    #[test]
    fn test_parallel_hash_v256(){
        assert_eq!(hex::encode(super::hashing::parallel_hash_v256(b"",b"Rust",32 as usize)), "0eb468f91701d9e48a25df40ab65e40bcf50b44b80ceb325b4f38971122456a1".to_string());
    }

    #[test]
    fn test_parallel_hash_xof_v128(){
        assert_eq!(hex::encode(super::hashing::parallel_hash_xof_v128(b"",b"Rust",32 as usize)), "38ac69589c1923e1e20a5a59676d2041b07e6b7cc1518ce75aaf6d84d1c693d5f91c2c19853a42846bad21502526dae488ab5022b1cfb3d4873a2f61f4d2409e".to_string());
    }
    #[test]
    fn test_parallel_hash_xof_v256(){
        assert_eq!(hex::encode(super::hashing::parallel_hash_xof_v256(b"",b"Rust",32 as usize)), "e3f296fffb24531f5768f26f958d9f6c45d9ce73592d8bb8b95e1edf575b8b639a315696c6f983dc96d7e385e4ae81a05c944c9438e5a06b39a0014e38d49568".to_string());
    }

    #[test]
    fn test_sha3_v224(){
        assert_eq!(hex::encode(super::hashing::sha3_v224(b"Rust")), "7c3347868eb2e602a9848b79be90da57872324b25641cfcfae6e79a2ca6a5647".to_string());
    }
    #[test]
    fn test_sha3_v256(){
        assert_eq!(hex::encode(super::hashing::sha3_v256(b"Rust")), "5c863b6ce037ef0e1fcc27c7f2254072a84297099852371d405fa100c5a47448".to_string());
    }
    #[test]
    fn test_sha3_v384(){
        assert_eq!(hex::encode(super::hashing::sha3_v384(b"Rust")), "c0aae1ea43ffa5125bfd088ff2e4863917ff8c1d1f81c5bea2b696d31b255354".to_string());
    }
    #[test]
    fn test_sha3_v512(){
        assert_eq!(hex::encode(super::hashing::sha3_v512(b"Rust")), "e486ca2748c2abe07cf76dc42661fd4bbf575e6d10db6256cf3f22a69a783577".to_string());
    }
    #[test]
    fn test_shake_v128(){
        assert_eq!(hex::encode(super::hashing::shake_v128(b"Rust")), "fc08c5bd23131bc3ef379ea21f55da0250f574e37e60fd0b6bcb30bf1f3f1217".to_string());
    }
    #[test]
    fn test_shake_v256(){
        assert_eq!(hex::encode(super::hashing::shake_v256(b"Rust")), "17b1fa274af90f48cd6949f8c54d104a01756532aa6220b8debb2a67e2ee5143".to_string());
    }
    #[test]
    fn test_tuple_hash_v128(){
        assert_eq!(hex::encode(super::hashing::tuple_hash_v128(b"",b"Rust")), "ff3dfb9b324b6952d20550482e31e37272b176761dc5dd1ead540aa08b4b1e9a".to_string());
    }
    #[test]
    fn test_tuple_hash_v256(){
        assert_eq!(hex::encode(super::hashing::tuple_hash_v256(b"",b"Rust")), "05dbf29cfdc76d1fdae0d0c56713c6433825596e51fe65b5591d6bf4d2877e8d".to_string());
    }

    #[test]
    fn test_tuple_hash_v128_xof(){
        assert_eq!(hex::encode(super::hashing::tuple_hash_v128_xof(b"",b"Rust")), "caaec204b9afb007e5a9e510f2d5a52d024a32b94cdd5539a563ca33d2118f309bba6d95641922af2f181c9261ffa408309d1959186bc4df325b85806bb90f58".to_string());
    }
    #[test]
    fn test_tuple_hash_v256_xof(){
        assert_eq!(hex::encode(super::hashing::tuple_hash_v256_xof(b"",b"Rust")), "73d4a97cb35fe3bb388dd6f655cac556b7ba61331519ebdfca36aea18e734e848d66b7a03b23154d3ae0f6284f95e4b2d66295c97e581e97c8ce92739697fbf3".to_string());
    }
}