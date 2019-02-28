use crypto_mac::generic_array::GenericArray;
use crypto_mac::{Mac, MacResult};

use std::marker::PhantomData;

/// A commitment to a particular `Secret`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Commit<T, M>
where
    M: Mac,
{
    value_type: PhantomData<T>,

    hash: GenericArray<u8, M::OutputSize>,
}

impl<T, M> Commit<T, M>
where
    M: Mac,
{
    pub(crate) fn into_mac(self) -> MacResult<M::OutputSize> {
        MacResult::new(self.hash)
    }

    pub(crate) fn new(key: &GenericArray<u8, M::KeySize>, data: &[u8]) -> Self {
        let mut mac = M::new(key);
        mac.input(data);

        Self {
            value_type: PhantomData,
            hash: mac.result().code(),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate hmac;
    extern crate sha2;

    use super::*;

    use bincode;

    use self::hmac::Hmac;
    use self::sha2::Sha256;

    #[test]
    fn serialize_deserialize() {
        struct Banana;

        let commit: Commit<Banana, Hmac<Sha256>> = Commit {
            value_type: PhantomData,
            hash: GenericArray::clone_from_slice(&[0; 32]),
        };

        let bytes = bincode::serialize(&commit).unwrap();

        let _: Commit<Banana, Hmac<Sha256>> = bincode::deserialize(&bytes).unwrap();
    }
}
