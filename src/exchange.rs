use commit::Commit;
use errors::*;
use reveal::Reveal;

use crypto_mac::Mac;

use std::cmp::Eq;
use std::collections::hash_map::{Entry, HashMap};
use std::hash::Hash;

/// Collects `Commit` instances from participants.
pub struct Exchange<T, I, M>
where
    M: Mac,
{
    commits: HashMap<I, Commit<T, M>>,
}

impl<T, I, M> Exchange<T, I, M>
where
    M: Mac,
    I: Eq + Hash,
{
    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            commits: HashMap::new(),
        }
    }

    /// Insert a `Commit` that came from a participant identified with `id`.
    ///
    ///  `id` is a unique identifier for a participant. An example might be a
    ///  `std::net::SocketAddr` for participants using sockets to communicate.
    pub fn insert(&mut self, id: I, commit: Commit<T, M>) -> Result<()> {
        match self.commits.entry(id) {
            Entry::Occupied(_) => bail!(ErrorKind::AlreadyInserted),
            Entry::Vacant(v) => {
                v.insert(commit);
            }
        }

        Ok(())
    }

    /// Indicate that all expected `Commit`s have been received.
    pub fn reveal(self) -> Result<Reveal<T, I, M>> {
        if self.commits.is_empty() {
            bail!(ErrorKind::Empty);
        } else {
            Ok(Reveal::new(self.commits))
        }
    }
}
