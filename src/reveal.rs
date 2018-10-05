use commit::Commit;
use errors::*;
use secret::Secret;

use crypto_mac::Mac;

use serde::Serialize;

use std::cmp::Eq;
use std::collections::hash_map::{Entry, HashMap};
use std::hash::Hash;
use std::ops::BitXorAssign;

/// Collects `Secret` instances from participants and verifies them against
/// previously collected `Commit` instances.
pub struct Reveal<T, I, M>
where
    M: Mac,
{
    commits: HashMap<I, Commit<T, M>>,
    secrets: HashMap<I, Secret<T, M>>,
}

impl<T, I, M> Reveal<T, I, M>
where
    M: Mac,
    I: Eq + Hash,
{
    pub(crate) fn new(commits: HashMap<I, Commit<T, M>>) -> Self {
        Self {
            commits,
            secrets: HashMap::new(),
        }
    }

    /// Insert a `Secret` that came from a participant identified with `id`.
    ///
    ///  `id` is a unique identifier for a participant. An example might be a
    ///  `std::net::SocketAddr` for participants using sockets to communicate.
    pub fn insert(&mut self, id: I, secret: Secret<T, M>) -> Result<()> {
        if !self.commits.contains_key(&id) {
            bail!(ErrorKind::NotPresent);
        }

        match self.secrets.entry(id) {
            // TODO: Return error instead of panicking
            Entry::Occupied(_) => bail!(ErrorKind::AlreadyInserted),
            Entry::Vacant(v) => {
                v.insert(secret);
            }
        }

        Ok(())
    }
}

impl<T, I, M> Reveal<T, I, M>
where
    M: Mac,
    I: Eq + Hash,
    T: Serialize + BitXorAssign,
{
    /// Get the random value generated by XORing all participants' contributions.
    ///
    /// # Security Information
    ///
    /// If the operation fails, the result will contain the identifiers given
    /// for the failing participants. If desired, the operation can be retried,
    /// but **without** those participants.
    ///
    /// Allowing malicious participants to continue retrying random value
    /// generation will allow them to abort any outcome that disadvantages them.
    pub fn get(mut self) -> RevealResult<T, I> {
        let mut failed = vec![];
        let mut result = None;

        for (id, commit) in self.commits.into_iter() {
            let secret = match self.secrets.remove(&id) {
                Some(x) => x,
                None => {
                    failed.push((RevealErrorKind::MissingSecret, id));
                    continue;
                }
            };

            let value = match secret.validate(commit) {
                Ok(v) => v,
                Err(_) => {
                    failed.push((RevealErrorKind::ValidationFailed, id));
                    continue;
                }
            };

            if let Some(ref mut v) = result {
                *v ^= value;
            } else {
                result = Some(value);
            }
        }

        if failed.is_empty() {
            if let Some(result) = result {
                Ok(result)
            } else {
                panic!("no errors and no result? This is a bug!");
            }
        } else {
            Err(RevealError::new(failed))
        }
    }
}
