use crypto_mac::generic_array::GenericArray;
use crypto_mac::{Mac, MacResult};

use std::marker::PhantomData;

/// A commitment to a particular `Secret`.
#[derive(Debug)]
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
