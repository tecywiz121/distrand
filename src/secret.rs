use commit::Commit;
use errors::*;

use crypto_mac::generic_array::GenericArray;
use crypto_mac::Mac;

use bincode;

use rand::{CryptoRng, Rng};

use serde::Serialize;

use typenum::Unsigned;

/// An individual participant's contribution to a randomly generated number.
#[derive(Debug, Serialize, Deserialize)]
pub struct Secret<T, M>
where
    M: Mac,
{
    value: T,
    key: GenericArray<u8, M::KeySize>,
}

impl<T, M> Secret<T, M>
where
    M: Mac,
{
    /// Returns a new secret with the given `value` and a randomly generated
    /// salt.
    pub fn new<R>(rng: &mut R, value: T) -> Self
    where
        R: Rng + CryptoRng,
    {
        let mut key = vec![0; M::KeySize::to_usize()];

        rng.fill_bytes(&mut key);

        Self {
            value,
            key: GenericArray::from_exact_iter(key).unwrap(),
        }
    }
}

impl<T, M> Secret<T, M>
where
    M: Mac,
    T: Serialize,
{
    /// Create a new `Commit` that can be used to validate a Secret during the
    /// reveal phase.
    pub fn commit(&self) -> Result<Commit<T, M>> {
        let bytes = bincode::serialize(&self.value)
            .map_err(|e| Error::with_chain(e, ErrorKind::Serialize))?;

        Ok(Commit::new(&self.key, &bytes))
    }

    pub(crate) fn validate(self, commit: Commit<T, M>) -> Result<T> {
        let mine = self.commit()?.into_mac();
        let other = commit.into_mac();

        if mine == other {
            Ok(self.value)
        } else {
            bail!("validation failed");
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate hmac;
    extern crate sha2;

    use self::hmac::Hmac;
    use self::sha2::Sha512;

    use bincode;

    use super::*;

    use rand::thread_rng;

    #[test]
    fn new() {
        let mut rng = thread_rng();

        let s: Secret<u32, Hmac<Sha512>> = Secret::new(&mut rng, 4);

        assert_eq!(s.value, 4);
        assert_eq!(s.key.len(), 128);
    }

    #[test]
    fn serialize_deserialize() {
        let secret: Secret<(), Hmac<Sha512>> = Secret {
            value: (),
            key: GenericArray::clone_from_slice(&[0; 128]),
        };

        let bytes = bincode::serialize(&secret).unwrap();

        let _: Secret<(), Hmac<Sha512>> = bincode::deserialize(&bytes).unwrap();
    }
}
